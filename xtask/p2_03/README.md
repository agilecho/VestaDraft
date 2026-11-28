# P2-03 local audit tools

Run `python3 -B -m unittest discover -s xtask/p2_03 -p test_audit_closeout.py -v`
for the ten new audit-tool checks. They use local synthetic data and temporary Git
repositories; they do not build or launch the product, original producers or guests.

`input_delta.py snapshot ROOT` prints all tracked and unignored file identities,
including untracked files and symlink targets. Deleted tracked files are absent.
`input_delta.py compare BEFORE.json AFTER.json` compares the complete path union and
prints added, removed, modified and unchanged paths. Both inputs must use the same
path namespace; normalize old absolute pins explicitly rather than dropping paths
through an intersection. Legacy path-to-SHA maps and snapshot descriptors are accepted.

`audit_readback.py --checker FILE --artifact FILE --zoom VALUE --output NEW.json`
uses the existing frozen YBa `verify_yba_product_roundtrip.py` checker and its pinned
inputs. It calls the deep byte/owner/geometry checks first, then reads the actual
bundle to serialize atoms/bonds/polyhedra, definition/resource identities, resource
bytes and the complete current-state SHA. The report also binds the reference graph,
primary artifact and checker. It launches no product. Exceptions propagate, a failure
does not publish a success report, and an existing report is never overwritten.

The historical G43 receipts and all frozen scripts remain unchanged. The new report
supplements their depth rather than changing their execution/version claims. The
full legacy helper suite also contains a supervisor test using macOS `ps`; inside
the network-denied macOS sandbox that setuid executable can be refused before the
synthetic worker result is observed. Keep that environmental failure visible rather
than altering the frozen monitor or claiming the entire helper suite passed.
