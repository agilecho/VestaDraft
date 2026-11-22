// Complete static module-closure inspection using the locked TypeScript AST.
// TypeScript 7.0.2 exposes its AST through unstable/*, not legacy createSourceFile.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {API} from 'typescript/unstable/sync';
import {SyntaxKind,isImportDeclaration,isExportDeclaration,isStringLiteral} from 'typescript/unstable/ast';

export function scanStaticClosure(entry,{root=process.cwd()}={}) {
  root=fs.realpathSync(root);
  const visited=new Set();
  const api=new API({cwd:root}); // Uses this exact package's bundled tsgo, never an external LSP.
  function walk(file) {
    file=path.resolve(file);
    assert.ok(file.startsWith(root+path.sep),'package escape '+file);
    assert.equal(fs.realpathSync(file),file,'symlink consumer '+file);
    if(visited.has(file))return;
    const text=fs.readFileSync(file,'utf8');
    // Preserve the previous Node global prohibition unchanged.
    assert.doesNotMatch(text,/\b(?:Buffer|process)\s*\.|\brequire\s*\(/,'Node global in '+file);
    visited.add(file);
    const snapshot=api.updateSnapshot({openFiles:[file]});
    const imports=[];
    try {
      const project=snapshot.getDefaultProjectForFile(file);
      assert.ok(project,'no AST project '+file);
      const source=project.program.getSourceFile(file);
      assert.ok(source,'no complete source AST '+file);
      assert.equal(source.text,text,'AST text differs from actual file '+file);
      assert.deepEqual(project.program.getSyntacticDiagnostics(file),[],'invalid syntax '+file);
      function visit(node) {
        let specifier;
        if(isImportDeclaration(node)||isExportDeclaration(node))specifier=node.moduleSpecifier;
        else if(node.kind===SyntaxKind.ImportEqualsDeclaration){
          const ref=node.moduleReference;
          if(ref.kind===SyntaxKind.ExternalModuleReference)specifier=ref.expression;
        } else if(node.kind===SyntaxKind.CallExpression&&node.expression.kind===SyntaxKind.ImportKeyword){
          assert.fail('dynamic import not admitted in pure consumer closure '+file);
        }
        if(specifier!==undefined){
          assert.ok(isStringLiteral(specifier),'non-literal module specifier '+file);
          imports.push(specifier.text);
        }
        // No truthy callback return: traverse every child, including same-line declarations.
        node.forEachChild(child=>{visit(child);});
      }
      visit(source);
    } finally {snapshot.dispose();}
    for(const specifier of imports){
      assert.ok(specifier.startsWith('./')||specifier.startsWith('../'),'external/Node dependency '+specifier);
      walk(path.resolve(path.dirname(file),specifier));
    }
  }
  try {walk(entry);return visited;} finally {api.close();}
}
