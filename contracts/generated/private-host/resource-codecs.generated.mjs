// Generated table; pure byte decoding grants no host authority.
import {codecRuntime} from '../../../scripts/lib/resource-runtime.mjs';
export const resourceBindings = [
  {
    "role": "OpenBundle",
    "descriptorDefinition": "OpenBundleResourceRef2",
    "mediaType": "application/vnd.vestacraft.native-open+json",
    "payloadSchema": {
      "$ref": "native-session-v2.schema.json#/$defs/OpenBundle"
    },
    "encoding": "strict-UTF8-JSON",
    "schemaGeneration": "I33",
    "permission": "descriptor/hash/type success never grants host/current-source authority",
    "envelopePointers": [
      "/$defs/Request/oneOf/8/properties/body/properties/bundle"
    ]
  },
  {
    "role": "InspectionPage",
    "descriptorDefinition": "InspectionPageResourceRef2",
    "mediaType": "application/vnd.vestacraft.native-inspection+json",
    "payloadSchema": {
      "$ref": "native-session-v2.schema.json#/$defs/InspectionPage"
    },
    "encoding": "strict-UTF8-JSON",
    "schemaGeneration": "I33",
    "permission": "descriptor/hash/type success never grants host/current-source authority",
    "envelopePointers": [
      "/$defs/Response/oneOf/8/properties/result/properties/page"
    ]
  },
  {
    "role": "HostAck",
    "descriptorDefinition": "HostAckResourceRef2",
    "mediaType": "application/vnd.vestacraft.host-ack+json",
    "payloadSchema": {
      "$ref": "native-session-v2.schema.json#/$defs/HostAck"
    },
    "encoding": "strict-UTF8-JSON",
    "schemaGeneration": "I33",
    "permission": "descriptor/hash/type success never grants host/current-source authority",
    "envelopePointers": [
      "/$defs/Request/oneOf/13/properties/body/properties/ack"
    ]
  },
  {
    "role": "HostNack",
    "descriptorDefinition": "HostNackResourceRef2",
    "mediaType": "application/vnd.vestacraft.host-nack+json",
    "payloadSchema": {
      "$ref": "native-session-v2.schema.json#/$defs/HostNack"
    },
    "encoding": "strict-UTF8-JSON",
    "schemaGeneration": "I33",
    "permission": "descriptor/hash/type success never grants host/current-source authority",
    "envelopePointers": [
      "/$defs/Request/oneOf/14/properties/body/properties/nack"
    ]
  },
  {
    "role": "CommandCompletion",
    "descriptorDefinition": "CommandCompletionResourceRef2",
    "mediaType": "application/vnd.vestacraft.command-outcome+json",
    "payloadSchema": {
      "$ref": "native-session-v2.schema.json#/$defs/CommandCompletion"
    },
    "encoding": "strict-UTF8-JSON",
    "schemaGeneration": "I33",
    "permission": "descriptor/hash/type success never grants host/current-source authority",
    "envelopePointers": [
      "/$defs/Response/oneOf/9/properties/result/properties/outcome"
    ]
  },
  {
    "role": "FailureNotice",
    "descriptorDefinition": "FailureNoticeResourceRef2",
    "mediaType": "application/vnd.vestacraft.engine-failure+json",
    "payloadSchema": {
      "$ref": "native-session-v2.schema.json#/$defs/FailureNotice"
    },
    "encoding": "strict-UTF8-JSON",
    "schemaGeneration": "I33",
    "permission": "descriptor/hash/type success never grants host/current-source authority",
    "envelopePointers": [
      "/$defs/Response/oneOf/6/properties/result/properties/diagnostics/oneOf/0",
      "/$defs/Response/oneOf/19/properties/engineFailure"
    ]
  }
]
;
export const schemaCatalog = {
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/contracts/native-session-v2.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:I33:native-session:2",
    "title": "NS/2 I33 explicit typed resource decoding and profile derivation successor",
    "description": "Request/Response and resource descriptors are separate named roots. All semantic/lifecycle/host trust rules mandatory; schema alone insufficient. Existing scientific177definitions/protocol3 untouched.",
    "$defs": {
      "Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$"
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "U64": {
        "type": "string",
        "pattern": "^(0|[1-9][0-9]*)$",
        "maxLength": 20
      },
      "Cell3": {
        "type": "object",
        "properties": {
          "lengthA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "anglesDeg": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "lengthA",
          "anglesDeg"
        ],
        "additionalProperties": false
      },
      "WorkRef": {
        "type": "object",
        "properties": {
          "path": {
            "type": "string"
          },
          "primarySha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          }
        },
        "required": [
          "path",
          "primarySha256"
        ],
        "additionalProperties": false
      },
      "DocumentAt": {
        "type": "object",
        "properties": {
          "documentId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "epoch": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          }
        },
        "required": [
          "documentId",
          "epoch",
          "revision"
        ],
        "additionalProperties": false
      },
      "ByteRef": {
        "type": "object",
        "properties": {
          "sha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "byteLength": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "mediaType": {
            "enum": [
              "application/octet-stream",
              "application/json",
              "application/vnd.vestacraft.native-open+json",
              "application/vnd.vestacraft.native-inspection+json",
              "application/vnd.vestacraft.save-bundle+json",
              "application/vnd.vestacraft.host-ack+json",
              "application/vnd.vestacraft.host-nack+json",
              "application/vnd.vestacraft.command-outcome+json",
              "application/vnd.vestacraft.engine-failure+json"
            ]
          }
        },
        "required": [
          "sha256",
          "byteLength",
          "mediaType"
        ],
        "additionalProperties": false
      },
      "Budget": {
        "type": "object",
        "properties": {
          "maxInputBytes": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "maxResidentBytes": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "maxFieldValues": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "maxMeshVertices": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "maxOutputBytes": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          }
        },
        "required": [
          "maxInputBytes",
          "maxResidentBytes",
          "maxFieldValues",
          "maxMeshVertices",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "Policy": {
        "type": "object",
        "properties": {
          "budget": {
            "$ref": "#/$defs/Budget"
          },
          "maxResponseCacheBytes": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "maxPendingUploads": {
            "type": "integer",
            "minimum": 1,
            "maximum": 1024
          },
          "maxPendingSnapshots": {
            "type": "integer",
            "minimum": 1,
            "maximum": 1024
          },
          "advanceWorkUnits": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "cancelGraceMs": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "hostCapabilitiesDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "maxPendingDomainCandidates": {
            "type": "integer",
            "minimum": 1,
            "maximum": 1024
          },
          "maxDomainResultBytes": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "domainCandidateRetentionMs": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          }
        },
        "required": [
          "budget",
          "maxResponseCacheBytes",
          "maxPendingUploads",
          "maxPendingSnapshots",
          "advanceWorkUnits",
          "cancelGraceMs",
          "hostCapabilitiesDigest",
          "maxPendingDomainCandidates",
          "maxDomainResultBytes",
          "domainCandidateRetentionMs"
        ],
        "additionalProperties": false
      },
      "Context": {
        "type": "object",
        "properties": {
          "documentId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "epoch": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "stateDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "dirty": {
            "type": "boolean"
          },
          "workRef": {
            "oneOf": [
              {
                "$ref": "#/$defs/WorkRef"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "documentId",
          "epoch",
          "revision",
          "stateDigest",
          "dirty",
          "workRef"
        ],
        "additionalProperties": false
      },
      "OpenBundle": {
        "type": "object",
        "properties": {
          "sourcePurpose": {
            "enum": [
              "current-work",
              "import"
            ]
          },
          "primary": {
            "$ref": "#/$defs/ByteRef"
          },
          "format": {
            "enum": [
              "vesta",
              "cif"
            ]
          },
          "dependencies": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "logicalPath": {
                  "type": "string"
                },
                "resource": {
                  "$ref": "#/$defs/ByteRef"
                }
              },
              "required": [
                "logicalPath",
                "resource"
              ],
              "additionalProperties": false
            },
            "minItems": 0
          },
          "workRef": {
            "oneOf": [
              {
                "$ref": "#/$defs/WorkRef"
              },
              {
                "type": "null"
              }
            ]
          },
          "allowDiagnosticImport": {
            "type": "boolean"
          }
        },
        "required": [
          "sourcePurpose",
          "primary",
          "format",
          "dependencies",
          "workRef",
          "allowDiagnosticImport"
        ],
        "additionalProperties": false
      },
      "PhaseRow": {
        "type": "object",
        "properties": {
          "phaseId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "label": {
            "type": "string"
          },
          "structurePresent": {
            "type": "boolean"
          },
          "cell": {
            "oneOf": [
              {
                "$ref": "#/$defs/Cell3"
              },
              {
                "type": "null"
              }
            ]
          },
          "volumeA3": {
            "oneOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "cellEditable": {
            "type": "boolean"
          },
          "siteCount": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          }
        },
        "required": [
          "phaseId",
          "label",
          "structurePresent",
          "cell",
          "volumeA3",
          "cellEditable",
          "siteCount"
        ],
        "additionalProperties": false
      },
      "SiteRow": {
        "type": "object",
        "properties": {
          "phaseId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "siteId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "label": {
            "type": "string"
          }
        },
        "required": [
          "phaseId",
          "siteId",
          "label"
        ],
        "additionalProperties": false
      },
      "InspectionPage": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "query": {
                "const": "phases"
              },
              "document": {
                "$ref": "#/$defs/DocumentAt"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "records": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/PhaseRow"
                },
                "minItems": 0
              }
            },
            "required": [
              "query",
              "document",
              "contextVersion",
              "records"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "query": {
                "const": "sites"
              },
              "document": {
                "$ref": "#/$defs/DocumentAt"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "records": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/SiteRow"
                },
                "minItems": 0
              }
            },
            "required": [
              "query",
              "document",
              "contextVersion",
              "records"
            ],
            "additionalProperties": false
          }
        ]
      },
      "Query": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "phases"
              },
              "limit": {
                "type": "integer",
                "minimum": 1,
                "maximum": 1000
              },
              "cursor": {
                "oneOf": [
                  {
                    "type": "string"
                  },
                  {
                    "type": "null"
                  }
                ]
              }
            },
            "required": [
              "kind",
              "limit",
              "cursor"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "sites"
              },
              "phaseId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "limit": {
                "type": "integer",
                "minimum": 1,
                "maximum": 1000
              },
              "cursor": {
                "oneOf": [
                  {
                    "type": "string"
                  },
                  {
                    "type": "null"
                  }
                ]
              }
            },
            "required": [
              "kind",
              "phaseId",
              "limit",
              "cursor"
            ],
            "additionalProperties": false
          }
        ]
      },
      "CoreInvocation": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "cell.set"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "cell": {
                    "$ref": "#/$defs/Cell3"
                  }
                },
                "required": [
                  "phaseId",
                  "cell"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "site.add"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "label": {
                    "type": "string"
                  },
                  "species": {
                    "type": "object",
                    "properties": {
                      "element": {
                        "type": "string"
                      }
                    },
                    "required": [
                      "element"
                    ],
                    "additionalProperties": false
                  },
                  "frac": {
                    "type": "array",
                    "items": {
                      "type": "number"
                    },
                    "minItems": 3,
                    "maxItems": 3
                  },
                  "occupancy": {
                    "type": "number",
                    "minimum": 0,
                    "maximum": 1
                  },
                  "uIsoA2": {
                    "type": "number",
                    "minimum": 0
                  }
                },
                "required": [
                  "phaseId",
                  "label",
                  "species",
                  "frac",
                  "occupancy",
                  "uIsoA2"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "site.setFractional"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "siteId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "frac": {
                    "type": "array",
                    "items": {
                      "type": "number"
                    },
                    "minItems": 3,
                    "maxItems": 3
                  }
                },
                "required": [
                  "phaseId",
                  "siteId",
                  "frac"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "history.undo"
              },
              "params": {
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "history.redo"
              },
              "params": {
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "display.setHidden"
              },
              "params": {
                "type": "object",
                "properties": {
                  "images": {
                    "type": "array",
                    "items": {
                      "type": "object",
                      "properties": {
                        "phase": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$"
                        },
                        "site": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$"
                        },
                        "op": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$"
                        },
                        "shift": {
                          "type": "array",
                          "items": {
                            "type": "integer",
                            "minimum": -2147483648,
                            "maximum": 2147483647
                          },
                          "minItems": 3,
                          "maxItems": 3
                        }
                      },
                      "required": [
                        "phase",
                        "site",
                        "op",
                        "shift"
                      ],
                      "additionalProperties": false
                    },
                    "minItems": 0
                  },
                  "hidden": {
                    "type": "boolean"
                  }
                },
                "required": [
                  "images",
                  "hidden"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "boundary.set"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "min": {
                    "type": "array",
                    "items": {
                      "type": "number"
                    },
                    "minItems": 3,
                    "maxItems": 3
                  },
                  "max": {
                    "type": "array",
                    "items": {
                      "type": "number"
                    },
                    "minItems": 3,
                    "maxItems": 3
                  }
                },
                "required": [
                  "phaseId",
                  "min",
                  "max"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "view.setCamera"
              },
              "params": {
                "type": "object",
                "properties": {
                  "rotation": {
                    "type": "array",
                    "items": {
                      "type": "array",
                      "items": {
                        "type": "number"
                      },
                      "minItems": 3,
                      "maxItems": 3
                    },
                    "minItems": 3,
                    "maxItems": 3
                  },
                  "projection": {
                    "enum": [
                      "parallel",
                      "perspective"
                    ]
                  },
                  "zoom": {
                    "type": "number",
                    "minimum": 0
                  }
                },
                "required": [
                  "rotation",
                  "projection",
                  "zoom"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "domain.prepare"
              },
              "params": {
                "type": "object",
                "properties": {
                  "request": {
                    "$ref": "scientific-composition.schema.json#/$defs/TypedJsonResource2"
                  },
                  "resultMode": {
                    "enum": [
                      "query",
                      "candidate"
                    ]
                  }
                },
                "required": [
                  "request",
                  "resultMode"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "domain.state.prepare"
              },
              "params": {
                "type": "object",
                "properties": {
                  "edit": {
                    "$ref": "scientific-composition.schema.json#/$defs/TypedJsonResource2"
                  }
                },
                "required": [
                  "edit"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "domain.apply"
              },
              "params": {
                "type": "object",
                "properties": {
                  "candidateId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "candidateDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  }
                },
                "required": [
                  "candidateId",
                  "candidateDigest"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "command",
              "params"
            ],
            "additionalProperties": false
          }
        ]
      },
      "SnapshotOptions": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "purpose": {
                "const": "save"
              },
              "format": {
                "const": "vesta"
              },
              "targetToken": {
                "type": "string"
              }
            },
            "required": [
              "purpose",
              "format",
              "targetToken"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "purpose": {
                "const": "saveAs"
              },
              "format": {
                "const": "vesta"
              },
              "targetToken": {
                "type": "string"
              }
            },
            "required": [
              "purpose",
              "format",
              "targetToken"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "purpose": {
                "const": "export"
              },
              "format": {
                "const": "cif"
              },
              "targetToken": {
                "type": "string"
              }
            },
            "required": [
              "purpose",
              "format",
              "targetToken"
            ],
            "additionalProperties": false
          }
        ]
      },
      "OperationRef": {
        "type": "object",
        "properties": {
          "registryGeneration": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "operationId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "recordSequence": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "publicationSequence": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "targetDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "capabilityDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "semanticEvidenceDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          }
        },
        "required": [
          "registryGeneration",
          "operationId",
          "recordSequence",
          "publicationSequence",
          "targetDigest",
          "capabilityDigest",
          "semanticEvidenceDigest"
        ],
        "additionalProperties": false
      },
      "HostAck": {
        "type": "object",
        "properties": {
          "snapshotId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "documentId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "epoch": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "stateDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "purpose": {
            "enum": [
              "save",
              "saveAs",
              "export"
            ]
          },
          "targetToken": {
            "type": "string"
          },
          "actualPrimarySha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "verifiedFiles": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "path": {
                  "type": "string"
                },
                "sha256": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{64}$"
                },
                "byteLength": {
                  "type": "string",
                  "pattern": "^(0|[1-9][0-9]*)$",
                  "maxLength": 20
                }
              },
              "required": [
                "path",
                "sha256",
                "byteLength"
              ],
              "additionalProperties": false
            },
            "minItems": 1
          },
          "output": {
            "$ref": "#/$defs/WorkRef"
          },
          "currentWorkRef": {
            "oneOf": [
              {
                "$ref": "#/$defs/WorkRef"
              },
              {
                "type": "null"
              }
            ]
          },
          "operation": {
            "$ref": "#/$defs/OperationRef"
          }
        },
        "required": [
          "snapshotId",
          "documentId",
          "epoch",
          "revision",
          "stateDigest",
          "purpose",
          "targetToken",
          "actualPrimarySha256",
          "verifiedFiles",
          "output",
          "currentWorkRef",
          "operation"
        ],
        "additionalProperties": false
      },
      "HostNack": {
        "type": "object",
        "properties": {
          "snapshotId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "documentId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "epoch": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "purpose": {
            "enum": [
              "save",
              "saveAs",
              "export"
            ]
          },
          "code": {
            "enum": [
              "E_EXTERNAL_CHANGE",
              "E_READ_ONLY",
              "E_QUOTA",
              "E_IO",
              "E_TIMEOUT",
              "E_TARGET_TOKEN_EXPIRED",
              "E_TARGET_TOKEN_INVALID",
              "E_CANCELLED"
            ]
          },
          "settlement": {
            "enum": [
              "not-published",
              "committed-unverified",
              "unknown"
            ]
          },
          "operation": {
            "oneOf": [
              {
                "$ref": "#/$defs/OperationRef"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "snapshotId",
          "documentId",
          "epoch",
          "revision",
          "purpose",
          "code",
          "settlement",
          "operation"
        ],
        "additionalProperties": false
      },
      "CloseProof": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "loaded-baseline"
              }
            },
            "required": [
              "kind"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "confirmed-snapshot"
              },
              "snapshotId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              }
            },
            "required": [
              "kind",
              "snapshotId"
            ],
            "additionalProperties": false
          }
        ]
      },
      "Request": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "op": {
                "const": "hello"
              },
              "policy": {
                "$ref": "#/$defs/Policy"
              },
              "expectedProfiles": {
                "type": "object",
                "properties": {
                  "nativeSchemaDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "coreRegistryDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "saveBundleProfileDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "hostCapabilitiesDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "scientificCatalogDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "assetCatalogDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "historyPolicyDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "preferencesSchemaDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "externalWorkflowSchemaDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  }
                },
                "required": [
                  "nativeSchemaDigest",
                  "coreRegistryDigest",
                  "saveBundleProfileDigest",
                  "hostCapabilitiesDigest",
                  "scientificCatalogDigest",
                  "assetCatalogDigest",
                  "historyPolicyDigest",
                  "preferencesSchemaDigest",
                  "externalWorkflowSchemaDigest"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "op",
              "policy",
              "expectedProfiles"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "op": {
                "const": "status"
              },
              "targetSeq": {
                "oneOf": [
                  {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  {
                    "type": "null"
                  }
                ]
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "op",
              "targetSeq"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "op": {
                "const": "cancel"
              },
              "targetSeq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "op",
              "targetSeq"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "resource.begin"
              },
              "body": {
                "type": "object",
                "properties": {
                  "expected": {
                    "$ref": "#/$defs/ByteRef"
                  }
                },
                "required": [
                  "expected"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "resource.chunk"
              },
              "body": {
                "type": "object",
                "properties": {
                  "uploadSeq": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "offset": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "bytesB64": {
                    "type": "string"
                  }
                },
                "required": [
                  "uploadSeq",
                  "offset",
                  "bytesB64"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "resource.commit"
              },
              "body": {
                "type": "object",
                "properties": {
                  "uploadSeq": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  }
                },
                "required": [
                  "uploadSeq"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "resource.read"
              },
              "body": {
                "type": "object",
                "properties": {
                  "resource": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "offset": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "maxBytes": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 65536
                  }
                },
                "required": [
                  "resource",
                  "offset",
                  "maxBytes"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "resource.release"
              },
              "body": {
                "type": "object",
                "properties": {
                  "resource": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  }
                },
                "required": [
                  "resource"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "open"
              },
              "body": {
                "type": "object",
                "properties": {
                  "bundle": {
                    "$ref": "#/$defs/OpenBundleResourceRef2"
                  }
                },
                "required": [
                  "bundle"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "new"
              },
              "body": {
                "type": "object",
                "properties": {
                  "cell": {
                    "$ref": "#/$defs/Cell3"
                  },
                  "label": {
                    "type": "string"
                  }
                },
                "required": [
                  "cell",
                  "label"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "inspect"
              },
              "body": {
                "type": "object",
                "properties": {
                  "document": {
                    "$ref": "#/$defs/DocumentAt"
                  },
                  "query": {
                    "$ref": "#/$defs/Query"
                  }
                },
                "required": [
                  "document",
                  "query"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "execute"
              },
              "body": {
                "type": "object",
                "properties": {
                  "document": {
                    "$ref": "#/$defs/DocumentAt"
                  },
                  "invocation": {
                    "$ref": "#/$defs/CoreInvocation"
                  }
                },
                "required": [
                  "document",
                  "invocation"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "snapshot"
              },
              "body": {
                "type": "object",
                "properties": {
                  "document": {
                    "$ref": "#/$defs/DocumentAt"
                  },
                  "options": {
                    "$ref": "#/$defs/SnapshotOptions"
                  }
                },
                "required": [
                  "document",
                  "options"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "save.confirm"
              },
              "body": {
                "type": "object",
                "properties": {
                  "ack": {
                    "$ref": "#/$defs/HostAckResourceRef2"
                  }
                },
                "required": [
                  "ack"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "save.reject"
              },
              "body": {
                "type": "object",
                "properties": {
                  "nack": {
                    "$ref": "#/$defs/HostNackResourceRef2"
                  }
                },
                "required": [
                  "nack"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "close"
              },
              "body": {
                "oneOf": [
                  {
                    "type": "object",
                    "properties": {
                      "document": {
                        "$ref": "#/$defs/DocumentAt"
                      },
                      "disposition": {
                        "const": "require-saved"
                      },
                      "proof": {
                        "$ref": "#/$defs/CloseProof"
                      }
                    },
                    "required": [
                      "document",
                      "disposition",
                      "proof"
                    ],
                    "additionalProperties": false
                  },
                  {
                    "type": "object",
                    "properties": {
                      "document": {
                        "$ref": "#/$defs/DocumentAt"
                      },
                      "disposition": {
                        "const": "discard"
                      }
                    },
                    "required": [
                      "document",
                      "disposition"
                    ],
                    "additionalProperties": false
                  }
                ]
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "op": {
                "const": "shutdown"
              },
              "body": {
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "seq",
              "contextVersion",
              "op",
              "body"
            ],
            "additionalProperties": false
          }
        ]
      },
      "Response": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "null"
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "hello"
              },
              "result": {
                "type": "object",
                "properties": {
                  "policyDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "profiles": {
                    "type": "object",
                    "properties": {
                      "nativeSchemaDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "coreRegistryDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "saveBundleProfileDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "hostCapabilitiesDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "scientificCatalogDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "assetCatalogDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "historyPolicyDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "preferencesSchemaDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      },
                      "externalWorkflowSchemaDigest": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$"
                      }
                    },
                    "required": [
                      "nativeSchemaDigest",
                      "coreRegistryDigest",
                      "saveBundleProfileDigest",
                      "hostCapabilitiesDigest",
                      "scientificCatalogDigest",
                      "assetCatalogDigest",
                      "historyPolicyDigest",
                      "preferencesSchemaDigest",
                      "externalWorkflowSchemaDigest"
                    ],
                    "additionalProperties": false
                  },
                  "maxFrameBytes": {
                    "const": 1048576
                  },
                  "maxRawChunkBytes": {
                    "const": 65536
                  },
                  "coreCommands": {
                    "type": "array",
                    "items": {
                      "enum": [
                        "cell.set",
                        "site.add",
                        "site.setFractional",
                        "history.undo",
                        "history.redo",
                        "display.setHidden",
                        "boundary.set",
                        "view.setCamera",
                        "domain.prepare",
                        "domain.state.prepare",
                        "domain.apply"
                      ]
                    },
                    "minItems": 0
                  },
                  "methods": {
                    "type": "array",
                    "items": {
                      "enum": [
                        "hello",
                        "status",
                        "cancel",
                        "resource.begin",
                        "resource.chunk",
                        "resource.commit",
                        "resource.read",
                        "resource.release",
                        "open",
                        "new",
                        "inspect",
                        "execute",
                        "snapshot",
                        "save.confirm",
                        "save.reject",
                        "close",
                        "shutdown"
                      ]
                    },
                    "minItems": 0
                  }
                },
                "required": [
                  "policyDigest",
                  "profiles",
                  "maxFrameBytes",
                  "maxRawChunkBytes",
                  "coreCommands",
                  "methods"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "resource.begin"
              },
              "result": {
                "type": "object",
                "properties": {
                  "uploadSeq": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "receivedBytes": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  }
                },
                "required": [
                  "uploadSeq",
                  "receivedBytes"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "resource.chunk"
              },
              "result": {
                "type": "object",
                "properties": {
                  "uploadSeq": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "receivedBytes": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  }
                },
                "required": [
                  "uploadSeq",
                  "receivedBytes"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "resource.commit"
              },
              "result": {
                "type": "object",
                "properties": {
                  "resource": {
                    "$ref": "#/$defs/ByteRef"
                  }
                },
                "required": [
                  "resource"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "resource.read"
              },
              "result": {
                "type": "object",
                "properties": {
                  "resource": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "offset": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "bytesB64": {
                    "type": "string"
                  },
                  "eof": {
                    "type": "boolean"
                  }
                },
                "required": [
                  "resource",
                  "offset",
                  "bytesB64",
                  "eof"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "resource.release"
              },
              "result": {
                "type": "object",
                "properties": {
                  "resource": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "transportLeaseReleased": {
                    "type": "boolean"
                  }
                },
                "required": [
                  "resource",
                  "transportLeaseReleased"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "open"
              },
              "result": {
                "type": "object",
                "properties": {
                  "context": {
                    "$ref": "#/$defs/Context"
                  },
                  "nextContextVersion": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "editable": {
                    "type": "boolean"
                  },
                  "diagnostics": {
                    "oneOf": [
                      {
                        "$ref": "#/$defs/FailureNoticeResourceRef2"
                      },
                      {
                        "type": "null"
                      }
                    ]
                  }
                },
                "required": [
                  "context",
                  "nextContextVersion",
                  "editable",
                  "diagnostics"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "new"
              },
              "result": {
                "type": "object",
                "properties": {
                  "context": {
                    "$ref": "#/$defs/Context"
                  },
                  "nextContextVersion": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  }
                },
                "required": [
                  "context",
                  "nextContextVersion",
                  "phaseId"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "inspect"
              },
              "result": {
                "type": "object",
                "properties": {
                  "document": {
                    "$ref": "#/$defs/DocumentAt"
                  },
                  "queryKind": {
                    "enum": [
                      "phases",
                      "sites"
                    ]
                  },
                  "page": {
                    "$ref": "#/$defs/InspectionPageResourceRef2"
                  },
                  "nextCursor": {
                    "oneOf": [
                      {
                        "type": "string"
                      },
                      {
                        "type": "null"
                      }
                    ]
                  }
                },
                "required": [
                  "document",
                  "queryKind",
                  "page",
                  "nextCursor"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "execute"
              },
              "result": {
                "type": "object",
                "properties": {
                  "context": {
                    "$ref": "#/$defs/Context"
                  },
                  "engineRequestId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "outcome": {
                    "$ref": "#/$defs/CommandCompletionResourceRef2"
                  }
                },
                "required": [
                  "context",
                  "engineRequestId",
                  "outcome"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "snapshot"
              },
              "result": {
                "type": "object",
                "properties": {
                  "snapshotId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "captured": {
                    "$ref": "#/$defs/Context"
                  },
                  "purpose": {
                    "enum": [
                      "save",
                      "saveAs",
                      "export"
                    ]
                  },
                  "targetTokenDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "bundle": {
                    "$ref": "#/$defs/ByteRef"
                  },
                  "sourceExpectationDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  }
                },
                "required": [
                  "snapshotId",
                  "captured",
                  "purpose",
                  "targetTokenDigest",
                  "bundle",
                  "sourceExpectationDigest"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "save.confirm"
              },
              "result": {
                "type": "object",
                "properties": {
                  "snapshotId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "savedDocument": {
                    "$ref": "#/$defs/DocumentAt"
                  },
                  "output": {
                    "$ref": "#/$defs/WorkRef"
                  },
                  "current": {
                    "$ref": "#/$defs/Context"
                  },
                  "publicationSequence": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  }
                },
                "required": [
                  "snapshotId",
                  "savedDocument",
                  "output",
                  "current",
                  "publicationSequence"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "save.reject"
              },
              "result": {
                "type": "object",
                "properties": {
                  "snapshotId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "settlement": {
                    "enum": [
                      "not-published",
                      "committed-unverified",
                      "unknown"
                    ]
                  },
                  "requiresResolution": {
                    "type": "boolean"
                  },
                  "current": {
                    "$ref": "#/$defs/Context"
                  }
                },
                "required": [
                  "snapshotId",
                  "settlement",
                  "requiresResolution",
                  "current"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "close"
              },
              "result": {
                "type": "object",
                "properties": {
                  "closedDocumentId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "closedEpoch": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "nextContextVersion": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  }
                },
                "required": [
                  "closedDocumentId",
                  "closedEpoch",
                  "nextContextVersion"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "shutdown"
              },
              "result": {
                "type": "object",
                "properties": {
                  "exitCode": {
                    "const": 0
                  }
                },
                "required": [
                  "exitCode"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "null"
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "status"
              },
              "result": {
                "type": "object",
                "properties": {
                  "highestAdmittedSeq": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "contextVersion": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  "context": {
                    "oneOf": [
                      {
                        "$ref": "#/$defs/Context"
                      },
                      {
                        "type": "null"
                      }
                    ]
                  },
                  "targetState": {
                    "enum": [
                      "session",
                      "not-admitted",
                      "active",
                      "finished",
                      "expired"
                    ]
                  },
                  "stage": {
                    "oneOf": [
                      {
                        "enum": [
                          "upload",
                          "queued",
                          "running",
                          "committed",
                          "snapshot-ready",
                          "saved",
                          "failed",
                          "cancelled"
                        ]
                      },
                      {
                        "type": "null"
                      }
                    ]
                  },
                  "result": {
                    "oneOf": [
                      {
                        "oneOf": [
                          {
                            "$ref": "#/$defs/CommandCompletionResourceRef2"
                          },
                          {
                            "allOf": [
                              {
                                "$ref": "#/$defs/ByteRef"
                              },
                              {
                                "properties": {
                                  "mediaType": {
                                    "const": "application/vnd.vestacraft.save-bundle+json"
                                  }
                                }
                              }
                            ]
                          }
                        ]
                      },
                      {
                        "type": "null"
                      }
                    ]
                  },
                  "receivedBytes": {
                    "oneOf": [
                      {
                        "type": "string",
                        "pattern": "^(0|[1-9][0-9]*)$",
                        "maxLength": 20
                      },
                      {
                        "type": "null"
                      }
                    ]
                  },
                  "committedRevision": {
                    "oneOf": [
                      {
                        "type": "string",
                        "pattern": "^(0|[1-9][0-9]*)$",
                        "maxLength": 20
                      },
                      {
                        "type": "null"
                      }
                    ]
                  },
                  "pendingSnapshots": {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  }
                },
                "required": [
                  "highestAdmittedSeq",
                  "contextVersion",
                  "context",
                  "targetState",
                  "stage",
                  "result",
                  "receivedBytes",
                  "committedRevision",
                  "pendingSnapshots"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "null"
              },
              "kind": {
                "const": "ok"
              },
              "op": {
                "const": "cancel"
              },
              "targetSeq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "result": {
                "type": "object",
                "properties": {
                  "outcome": {
                    "enum": [
                      "cancelled-before-commit",
                      "too-late",
                      "already-finished",
                      "not-admitted",
                      "expired"
                    ]
                  },
                  "committedRevision": {
                    "oneOf": [
                      {
                        "type": "string",
                        "pattern": "^(0|[1-9][0-9]*)$",
                        "maxLength": 20
                      },
                      {
                        "type": "null"
                      }
                    ]
                  }
                },
                "required": [
                  "outcome",
                  "committedRevision"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "targetSeq",
              "result"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "accepted"
              },
              "op": {
                "enum": [
                  "execute",
                  "snapshot"
                ]
              },
              "jobId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "jobId"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "kind": {
                "const": "error"
              },
              "sessionId": {
                "oneOf": [
                  {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  {
                    "type": "null"
                  }
                ]
              },
              "contextVersion": {
                "oneOf": [
                  {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  {
                    "type": "null"
                  }
                ]
              },
              "seq": {
                "oneOf": [
                  {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  {
                    "type": "null"
                  }
                ]
              },
              "code": {
                "enum": [
                  "N_FRAME",
                  "N_UTF8",
                  "N_PROTOCOL",
                  "N_SCHEMA",
                  "N_POLICY",
                  "N_UNINITIALIZED",
                  "N_ALREADY_INITIALIZED",
                  "N_STALE_SESSION",
                  "N_SEQUENCE_GAP",
                  "N_REQUEST_EXPIRED",
                  "N_REPLAY_MISMATCH",
                  "N_CONTEXT_CONFLICT",
                  "N_BUSY",
                  "N_SCOPE_UNSUPPORTED",
                  "N_PROFILE_UNAVAILABLE",
                  "N_RESOURCE_LIMIT",
                  "N_RESOURCE_MISSING",
                  "N_DIGEST",
                  "N_UPLOAD_OFFSET",
                  "N_UPLOAD_STATE",
                  "N_CLOSE_REQUIRED",
                  "N_SNAPSHOT_UNKNOWN",
                  "N_HOST_ONLY",
                  "N_INCOMPLETE_WORK",
                  "N_INTERNAL",
                  "N_SNAPSHOT_CONFLICT"
                ]
              },
              "admitted": {
                "type": "boolean"
              },
              "fatal": {
                "type": "boolean"
              },
              "highestAdmittedSeq": {
                "oneOf": [
                  {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  {
                    "type": "null"
                  }
                ]
              },
              "message": {
                "type": "string",
                "maxLength": 1024
              }
            },
            "required": [
              "protocol",
              "kind",
              "sessionId",
              "contextVersion",
              "seq",
              "code",
              "admitted",
              "fatal",
              "highestAdmittedSeq",
              "message"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.native-session/2"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "contextVersion": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "seq": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "kind": {
                "const": "failed"
              },
              "op": {
                "enum": [
                  "execute",
                  "snapshot"
                ]
              },
              "code": {
                "enum": [
                  "E_SCHEMA",
                  "E_DOCUMENT_NOT_FOUND",
                  "E_STALE_EPOCH",
                  "E_OBJECT_NOT_FOUND",
                  "E_REVISION_CONFLICT",
                  "E_STALE_CURSOR",
                  "E_IDEMPOTENCY_CONFLICT",
                  "E_REQUEST_EXPIRED",
                  "E_NO_HISTORY",
                  "E_DOMAIN",
                  "E_RESOURCE_MISSING",
                  "E_RESOURCE_LIMIT",
                  "E_FORMAT_UNSUPPORTED",
                  "E_FORMAT_LOSS",
                  "E_RULE_UNFROZEN",
                  "E_NOT_IMPLEMENTED",
                  "E_NONATOMIC_EFFECT",
                  "E_CANCELLED",
                  "E_RENDER_FAILED",
                  "E_EXTERNAL_CHANGE",
                  "E_READ_ONLY",
                  "E_QUOTA",
                  "E_IO",
                  "E_TIMEOUT",
                  "E_REFERENCE_FAILURE",
                  "E_TARGET_TOKEN_EXPIRED",
                  "E_TARGET_TOKEN_INVALID"
                ]
              },
              "committedRevision": {
                "oneOf": [
                  {
                    "type": "string",
                    "pattern": "^(0|[1-9][0-9]*)$",
                    "maxLength": 20
                  },
                  {
                    "type": "null"
                  }
                ]
              },
              "engineFailure": {
                "$ref": "#/$defs/FailureNoticeResourceRef2"
              }
            },
            "required": [
              "protocol",
              "sessionId",
              "contextVersion",
              "seq",
              "kind",
              "op",
              "code",
              "committedRevision",
              "engineFailure"
            ],
            "additionalProperties": false
          }
        ]
      },
      "Warning": {
        "type": "object",
        "properties": {
          "code": {
            "const": "W_REFERENCE_DEFECT"
          },
          "featureId": {
            "type": "string",
            "pattern": "^VF-[0-9]{4}$"
          },
          "referenceVersion": {
            "type": "string"
          },
          "evidenceRef": {
            "type": "string"
          },
          "ruleId": {
            "type": "string"
          },
          "impact": {
            "type": "string"
          }
        },
        "required": [
          "code",
          "featureId",
          "referenceVersion",
          "evidenceRef",
          "ruleId",
          "impact"
        ],
        "additionalProperties": false
      },
      "ChangeSet": {
        "type": "object",
        "properties": {
          "before": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "after": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "digest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "changedObjects": {
            "type": "array",
            "items": {
              "type": "string",
              "pattern": "^[0-9a-f]{32}$"
            },
            "minItems": 0
          },
          "invalidated": {
            "type": "array",
            "items": {
              "enum": [
                "domain",
                "scene",
                "field",
                "render"
              ]
            },
            "minItems": 0
          }
        },
        "required": [
          "before",
          "after",
          "digest",
          "changedObjects",
          "invalidated"
        ],
        "additionalProperties": false
      },
      "CommandCompletion": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "cell.set"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "site.add"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "object",
                "properties": {
                  "siteId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  }
                },
                "required": [
                  "siteId"
                ],
                "additionalProperties": false
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "site.setFractional"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "history.undo"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "history.redo"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "display.setHidden"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "boundary.set"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "view.setCamera"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "type": "null"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "domain.prepare"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "$ref": "scientific-composition.schema.json#/$defs/PrepareDomainResult2"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "domain.state.prepare"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "$ref": "scientific-composition.schema.json#/$defs/PrepareStateResult2"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "command": {
                "const": "domain.apply"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "revision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "stateDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "changeSet": {
                "$ref": "#/$defs/ChangeSet"
              },
              "result": {
                "$ref": "scientific-composition.schema.json#/$defs/AppliedDomainResult2"
              },
              "warnings": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Warning"
                },
                "minItems": 0
              },
              "history": {
                "$ref": "#/$defs/HistorySummary2"
              }
            },
            "required": [
              "command",
              "requestId",
              "documentId",
              "epoch",
              "revision",
              "stateDigest",
              "changeSet",
              "result",
              "warnings",
              "history"
            ],
            "additionalProperties": false
          }
        ]
      },
      "FailureNotice": {
        "type": "object",
        "properties": {
          "requestId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "code": {
            "enum": [
              "E_SCHEMA",
              "E_DOCUMENT_NOT_FOUND",
              "E_STALE_EPOCH",
              "E_OBJECT_NOT_FOUND",
              "E_REVISION_CONFLICT",
              "E_STALE_CURSOR",
              "E_IDEMPOTENCY_CONFLICT",
              "E_REQUEST_EXPIRED",
              "E_NO_HISTORY",
              "E_DOMAIN",
              "E_RESOURCE_MISSING",
              "E_RESOURCE_LIMIT",
              "E_FORMAT_UNSUPPORTED",
              "E_FORMAT_LOSS",
              "E_RULE_UNFROZEN",
              "E_NOT_IMPLEMENTED",
              "E_NONATOMIC_EFFECT",
              "E_CANCELLED",
              "E_RENDER_FAILED",
              "E_EXTERNAL_CHANGE",
              "E_READ_ONLY",
              "E_QUOTA",
              "E_IO",
              "E_TIMEOUT",
              "E_REFERENCE_FAILURE",
              "E_TARGET_TOKEN_EXPIRED",
              "E_TARGET_TOKEN_INVALID"
            ]
          },
          "stage": {
            "enum": [
              "schema",
              "identity",
              "capability",
              "prepare",
              "execute",
              "commit",
              "encode"
            ]
          },
          "message": {
            "type": "string",
            "maxLength": 1024
          },
          "retryable": {
            "type": "boolean"
          },
          "committedRevision": {
            "oneOf": [
              {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              {
                "type": "null"
              }
            ]
          },
          "recovery": {
            "enum": [
              "refresh-context",
              "inspect-committed",
              "query-same-operation",
              "explicit-new-request",
              "none"
            ]
          }
        },
        "required": [
          "requestId",
          "code",
          "stage",
          "message",
          "retryable",
          "committedRevision",
          "recovery"
        ],
        "additionalProperties": false
      },
      "HistorySummary2": {
        "$ref": "history-effects-wire.schema.json#/$defs/HistoryAwareCommitSummary"
      },
      "OpenBundleResourceRef2": {
        "allOf": [
          {
            "$ref": "#/$defs/ByteRef"
          },
          {
            "properties": {
              "mediaType": {
                "const": "application/vnd.vestacraft.native-open+json"
              }
            }
          }
        ],
        "x-resourcePayloadSchema": {
          "$ref": "#/$defs/OpenBundle"
        }
      },
      "InspectionPageResourceRef2": {
        "allOf": [
          {
            "$ref": "#/$defs/ByteRef"
          },
          {
            "properties": {
              "mediaType": {
                "const": "application/vnd.vestacraft.native-inspection+json"
              }
            }
          }
        ],
        "x-resourcePayloadSchema": {
          "$ref": "#/$defs/InspectionPage"
        }
      },
      "HostAckResourceRef2": {
        "allOf": [
          {
            "$ref": "#/$defs/ByteRef"
          },
          {
            "properties": {
              "mediaType": {
                "const": "application/vnd.vestacraft.host-ack+json"
              }
            }
          }
        ],
        "x-resourcePayloadSchema": {
          "$ref": "#/$defs/HostAck"
        }
      },
      "HostNackResourceRef2": {
        "allOf": [
          {
            "$ref": "#/$defs/ByteRef"
          },
          {
            "properties": {
              "mediaType": {
                "const": "application/vnd.vestacraft.host-nack+json"
              }
            }
          }
        ],
        "x-resourcePayloadSchema": {
          "$ref": "#/$defs/HostNack"
        }
      },
      "CommandCompletionResourceRef2": {
        "allOf": [
          {
            "$ref": "#/$defs/ByteRef"
          },
          {
            "properties": {
              "mediaType": {
                "const": "application/vnd.vestacraft.command-outcome+json"
              }
            }
          }
        ],
        "x-resourcePayloadSchema": {
          "$ref": "#/$defs/CommandCompletion"
        }
      },
      "FailureNoticeResourceRef2": {
        "allOf": [
          {
            "$ref": "#/$defs/ByteRef"
          },
          {
            "properties": {
              "mediaType": {
                "const": "application/vnd.vestacraft.engine-failure+json"
              }
            }
          }
        ],
        "x-resourcePayloadSchema": {
          "$ref": "#/$defs/FailureNotice"
        }
      }
    },
    "$ref": "#/$defs/Request",
    "x-predecessor": {
      "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-071145-native-session/contracts/native-session.schema.json",
      "sha256": "bfcae2e333ce537a0be47cef7008558de78909a7e50627c492abf11003c3198f"
    },
    "x-designOnly": true,
    "x-requiredSemanticChecks": [
      "all u64 canonical strings parsed exactly and<=2^64-1",
      "typed JSON resource original bytes/hash/size, strict duplicate-key/nonfinite/unknown-field rejection before exact OperationRequest2 or ScientificStateEdit2 decode",
      "resource staging hash is not current-source authority or host privilege",
      "same seq originalbytes retries, same process/session/actualdocument/epoch/context semantics inheritedNS1",
      "every completion history document/epoch/revision/liveContentDigest equal actual command outcome; nochange preserves history",
      "candidate IDs/digests bind exact immutable typed result+sourcecontext+operation/profile+budgetpolicy; no unpairednulls; partial/sourcefailure never apply",
      "maxPendingDomainCandidates/maxDomainResultBytes/retentionMs are mandatory positive actualhost policy limits; counts/bytes also within Budget, expiry releases candidate/job retains but cachedterminal never reexecutes",
      "domain.apply accepts only actual owned complete unexpired candidate at capturedrevision; materialized field/mesh/state publication full validation then one atomiccommit",
      "domain.prepare/query never marks saved or invokes shell/external-process/preferences owner; privileged EX3/AL5 routes use authenticated existing host separately",
      "resource role derived from envelope pointer and admitted operation, never inferred from caller media alone; resolve same-session owned resource, limits, length/hash, strict UTF8 and JSON then exact role schema before Engine/SaveTracker; x-resourcePayloadSchema is mandatory generator/decoder input, not automatic JSONSchema evaluation of external bytes",
      "CommandCompletion command must equal admitted invocation; history exact command outcome and wire u64/current-owner invariants; all11 variants verified; HostAck/Nack require captured pending snapshot/privatehost/HS receipts after shape validation; resource decode failure never settles Saved",
      "resource.begin/commit/raw primary/dependency descriptors do not decode arbitrary bytes as JSON. Snapshot SaveBundle keeps pinned FC/I31 external producer contract; status chooses the admitted original operation. No inline large-payload fallback; max frame/chunk and resource leases unchanged"
    ],
    "x-resourceBindings": [
      {
        "role": "OpenBundle",
        "descriptor": {
          "$ref": "#/$defs/OpenBundleResourceRef2"
        },
        "payload": {
          "$ref": "#/$defs/OpenBundle"
        }
      },
      {
        "role": "InspectionPage",
        "descriptor": {
          "$ref": "#/$defs/InspectionPageResourceRef2"
        },
        "payload": {
          "$ref": "#/$defs/InspectionPage"
        }
      },
      {
        "role": "HostAck",
        "descriptor": {
          "$ref": "#/$defs/HostAckResourceRef2"
        },
        "payload": {
          "$ref": "#/$defs/HostAck"
        }
      },
      {
        "role": "HostNack",
        "descriptor": {
          "$ref": "#/$defs/HostNackResourceRef2"
        },
        "payload": {
          "$ref": "#/$defs/HostNack"
        }
      },
      {
        "role": "CommandCompletion",
        "descriptor": {
          "$ref": "#/$defs/CommandCompletionResourceRef2"
        },
        "payload": {
          "$ref": "#/$defs/CommandCompletion"
        }
      },
      {
        "role": "FailureNotice",
        "descriptor": {
          "$ref": "#/$defs/FailureNoticeResourceRef2"
        },
        "payload": {
          "$ref": "#/$defs/FailureNotice"
        }
      }
    ],
    "x-profileDerivationContract": "profile-derivation.contract.json",
    "x-contractGeneration": "I33"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/contracts/scientific-composition.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:I33:scientific-composition:2",
    "$defs": {
      "Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$"
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "U64": {
        "type": "string",
        "pattern": "^(0|[1-9][0-9]*)$",
        "maxLength": 20
      },
      "OperationRequest2": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.ResolveNativePlacements"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/ResolveNativePlacementsRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeGroup"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeGroupRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeWyckoff"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeWyckoffRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeDimensionD"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeDimensionDRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.ApplyNativeDomainParameters"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/ApplyNativeDomainParametersRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeADPAndUncertainty"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeADPAndUncertaintyRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeMerge"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeMergeRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeBondValence"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeBVRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativePlane"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativePlaneRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeShape"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeShapeRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeCamera"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeCameraRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.PrepareNativeFourierUi"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeFourierUiInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.CalcNativeGridNo"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeGridNoInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.CalcNativeTchWidth"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeTchWidthInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.AccumulateNativePowderPattern"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePowderAccumulateInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.MaterializeOrderedScalarExpression"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/OrderedExpressionInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ResampleAffineTrilinear"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Math_FSResampleAffineInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.LinearCombineFields"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Math_FSLinearCombineFieldsInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SampleCartesianLine"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Math_FSCartesianLineInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.LinearCombineCurves"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Math_FSLinearCombineCurvesInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.EvaluateNativeDispersion"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeDispersionInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.CalculateNativeStructureFactors"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeScatteringInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SynthesizeNativeFourierField"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeFourierFieldInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.GenerateNativeModelField"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeModelFieldInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.TransformNativeVolume"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeVolumeTransformInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SearchNativePeaksAndIntegrate"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePeakSearchInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.IntegrateNativeVoronoi"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeVoronoiInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativeIsosurfaceMesh"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeMeshInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ConvertNativeElectronicEnergy"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeConversionInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SynthesizeNativePowderPattern"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePowderFullInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ResolveNativeDefaultIsovalue"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeDefaultIsoInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ReadNativeReflectionSource"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeReflectionReadInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SampleNativeFractionalPoint"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativePointSampleInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ExportNativeLineProfile"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativeLineProfileInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.GenerateNativeContourLevels"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativeContourLevelsInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativeContourSegments"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativeContourSegmentsInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ExecuteNativePolyIntegralMenu"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativePolyIntegralInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativeObliqueSlice"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Slice_NativeObliqueSliceInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativePeriodicSlice"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Slice_NativePeriodicSliceInput"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "surface.RenderNativeIsosurfacePacket"
              },
              "input": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/render-consumption/render-packet.schema.json#/$defs/RenderNativeIsosurfacePacketRequest"
              }
            },
            "required": [
              "operation",
              "input"
            ],
            "additionalProperties": false
          }
        ]
      },
      "OperationResult2": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.ResolveNativePlacements"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/ResolveNativePlacementsResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeGroup"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeGroupResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeWyckoff"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeWyckoffResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeDimensionD"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeDimensionDResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.ApplyNativeDomainParameters"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/ApplyNativeDomainParametersResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeADPAndUncertainty"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeADPAndUncertaintyResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeMerge"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeMergeResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeBondValence"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeBVResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativePlane"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativePlaneResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeShape"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeShapeResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "sg2.NativeCamera"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeCameraResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.PrepareNativeFourierUi"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePreparedFourierOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.CalcNativeGridNo"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeGridNoOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.CalcNativeTchWidth"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeTchWidthOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.AccumulateNativePowderPattern"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePowderAccumulateOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.MaterializeOrderedScalarExpression"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/OrderedExpressionOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ResampleAffineTrilinear"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/FS1_FSFieldOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.LinearCombineFields"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/FS1_FSFieldOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SampleCartesianLine"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Math_FSCartesianLineOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.LinearCombineCurves"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Math_FSLinearCombineCurvesOutput"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.EvaluateNativeDispersion"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeDispersionOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.CalculateNativeStructureFactors"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeScatteringOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SynthesizeNativeFourierField"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeFullFieldOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.GenerateNativeModelField"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeModelFieldOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.TransformNativeVolume"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeVolumeTransformOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SearchNativePeaksAndIntegrate"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePeakVoronoiOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.IntegrateNativeVoronoi"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePeakVoronoiOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativeIsosurfaceMesh"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeMeshOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ConvertNativeElectronicEnergy"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeConversionOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SynthesizeNativePowderPattern"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativePowderFullOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ResolveNativeDefaultIsovalue"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeDefaultIsoOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ReadNativeReflectionSource"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/NativeReflectionReadOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.SampleNativeFractionalPoint"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativePointSampleOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ExportNativeLineProfile"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativeLineProfileOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.GenerateNativeContourLevels"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativeContourLevelsOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativeContourSegments"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativeContourSegmentsOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.ExecuteNativePolyIntegralMenu"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Sampling_NativePolyIntegralOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativeObliqueSlice"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Slice_NativeObliqueSliceOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "fs2.BuildNativePeriodicSlice"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Slice_NativePeriodicSliceOutputOrFailure"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "surface.RenderNativeIsosurfacePacket"
              },
              "output": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/render-consumption/render-packet.schema.json#/$defs/RenderNativeIsosurfacePacketResponse"
              }
            },
            "required": [
              "operation",
              "output"
            ],
            "additionalProperties": false
          }
        ]
      },
      "HistoryAwareCommitSummaryWire2": {
        "$ref": "history-effects-wire.schema.json#/$defs/HistoryAwareCommitSummary"
      },
      "TypedJsonResource2": {
        "type": "object",
        "properties": {
          "sha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "byteLength": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "mediaType": {
            "const": "application/json"
          }
        },
        "required": [
          "sha256",
          "byteLength",
          "mediaType"
        ],
        "additionalProperties": false
      },
      "ScientificStateEdit2": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "phase-placements"
              },
              "targetDocumentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/PhasePlacementSet"
              }
            },
            "required": [
              "component",
              "targetDocumentId",
              "state"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "native-phase-state"
              },
              "targetPhaseId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeDimensionDState"
              }
            },
            "required": [
              "component",
              "targetPhaseId",
              "state"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "group-setting"
              },
              "targetPhaseId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/GroupSettingState"
              }
            },
            "required": [
              "component",
              "targetPhaseId",
              "state"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "native-camera"
              },
              "targetSceneId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeCamera"
              }
            },
            "required": [
              "component",
              "targetSceneId",
              "state"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "occupation-domain-parameters"
              },
              "targetPhaseId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/ApplyNativeDomainParametersRequest"
              }
            },
            "required": [
              "component",
              "targetPhaseId",
              "state"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "shape"
              },
              "targetPhaseId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativeShapeRequest"
              }
            },
            "required": [
              "component",
              "targetPhaseId",
              "state"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "component": {
                "const": "plane"
              },
              "targetPhaseId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "state": {
                "$ref": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json#/$defs/NativePlaneRequest"
              }
            },
            "required": [
              "component",
              "targetPhaseId",
              "state"
            ],
            "additionalProperties": false
          }
        ]
      },
      "PrepareDomainResult2": {
        "type": "object",
        "properties": {
          "operation": {
            "enum": [
              "sg2.ResolveNativePlacements",
              "sg2.NativeGroup",
              "sg2.NativeWyckoff",
              "sg2.NativeDimensionD",
              "sg2.ApplyNativeDomainParameters",
              "sg2.NativeADPAndUncertainty",
              "sg2.NativeMerge",
              "sg2.NativeBondValence",
              "sg2.NativePlane",
              "sg2.NativeShape",
              "sg2.NativeCamera",
              "fs2.PrepareNativeFourierUi",
              "fs2.CalcNativeGridNo",
              "fs2.CalcNativeTchWidth",
              "fs2.AccumulateNativePowderPattern",
              "fs2.MaterializeOrderedScalarExpression",
              "fs2.ResampleAffineTrilinear",
              "fs2.LinearCombineFields",
              "fs2.SampleCartesianLine",
              "fs2.LinearCombineCurves",
              "fs2.EvaluateNativeDispersion",
              "fs2.CalculateNativeStructureFactors",
              "fs2.SynthesizeNativeFourierField",
              "fs2.GenerateNativeModelField",
              "fs2.TransformNativeVolume",
              "fs2.SearchNativePeaksAndIntegrate",
              "fs2.IntegrateNativeVoronoi",
              "fs2.BuildNativeIsosurfaceMesh",
              "fs2.ConvertNativeElectronicEnergy",
              "fs2.SynthesizeNativePowderPattern",
              "fs2.ResolveNativeDefaultIsovalue",
              "fs2.ReadNativeReflectionSource",
              "fs2.SampleNativeFractionalPoint",
              "fs2.ExportNativeLineProfile",
              "fs2.GenerateNativeContourLevels",
              "fs2.BuildNativeContourSegments",
              "fs2.ExecuteNativePolyIntegralMenu",
              "fs2.BuildNativeObliqueSlice",
              "fs2.BuildNativePeriodicSlice",
              "surface.RenderNativeIsosurfacePacket"
            ]
          },
          "output": {
            "$ref": "#/$defs/TypedJsonResource2"
          },
          "candidateId": {
            "oneOf": [
              {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              {
                "type": "null"
              }
            ]
          },
          "candidateDigest": {
            "oneOf": [
              {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              {
                "type": "null"
              }
            ]
          },
          "complete": {
            "type": "boolean"
          }
        },
        "required": [
          "operation",
          "output",
          "candidateId",
          "candidateDigest",
          "complete"
        ],
        "additionalProperties": false
      },
      "PrepareStateResult2": {
        "type": "object",
        "properties": {
          "candidateId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "candidateDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "validatedState": {
            "$ref": "#/$defs/TypedJsonResource2"
          }
        },
        "required": [
          "candidateId",
          "candidateDigest",
          "validatedState"
        ],
        "additionalProperties": false
      },
      "AppliedDomainResult2": {
        "type": "object",
        "properties": {
          "candidateId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "candidateDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "applied": {
            "type": "boolean"
          }
        },
        "required": [
          "candidateId",
          "candidateDigest",
          "applied"
        ],
        "additionalProperties": false
      }
    },
    "title": "Explicit closed I32 scientific operation/resource/state composition; never arbitrary options JSON"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/contracts/history-effects-wire.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:design:I32-history-effects-wire:2",
    "title": "I32 wire projection; u64 are canonical decimal strings",
    "$defs": {
      "InternalHistoryEffectDecision": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "record"
              },
              "registryRowDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "producerProfileDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              }
            },
            "required": [
              "kind",
              "registryRowDigest",
              "producerProfileDigest"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "preserve-history-cursor"
              },
              "registryRowDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "producerProfileDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "animationSessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "tickSequence": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "x-semantic-range": "u64, value<=18446744073709551615; parse without JavaScript Number"
              },
              "mutationMaskDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              }
            },
            "required": [
              "kind",
              "registryRowDigest",
              "producerProfileDigest",
              "animationSessionId",
              "tickSequence",
              "mutationMaskDigest"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "navigate-history"
              },
              "registryRowDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "producerProfileDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "direction": {
                "enum": [
                  "undo",
                  "redo"
                ]
              },
              "steps": {
                "const": 1
              }
            },
            "required": [
              "kind",
              "registryRowDigest",
              "producerProfileDigest",
              "direction",
              "steps"
            ],
            "additionalProperties": false
          }
        ]
      },
      "HistoryAwareCommitSummary": {
        "type": "object",
        "properties": {
          "documentId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "epoch": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "x-semantic-range": "u64, value<=18446744073709551615; parse without JavaScript Number"
          },
          "liveContentDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "historyGeneration": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "x-semantic-range": "u64, value<=18446744073709551615; parse without JavaScript Number"
          },
          "historyCount": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "x-semantic-range": "u64, value<=18446744073709551615; parse without JavaScript Number"
          },
          "historyCursor": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "x-semantic-range": "u64, value<=18446744073709551615; parse without JavaScript Number"
          },
          "cursorSnapshotDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "historyEffect": {
            "enum": [
              "record",
              "preserve-history-cursor",
              "navigate-history",
              "unchanged"
            ]
          },
          "undoAvailable": {
            "type": "boolean"
          },
          "redoAvailable": {
            "type": "boolean"
          }
        },
        "required": [
          "documentId",
          "epoch",
          "revision",
          "liveContentDigest",
          "historyGeneration",
          "historyCount",
          "historyCursor",
          "cursorSnapshotDigest",
          "historyEffect",
          "undoAvailable",
          "redoAvailable"
        ],
        "additionalProperties": false
      }
    },
    "x-predecessor": {
      "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/application/history-effects.schema.json",
      "sha256": "f0283291fb4303ca4c629edc5424599e05487c981407011e58c1e77e288a16ca",
      "role": "sealed internal logical integer schema, not a change to NS1 wire"
    },
    "x-convertedU64Fields": [
      "/InternalHistoryEffectDecision/oneOf/1/properties/tickSequence",
      "/HistoryAwareCommitSummary/properties/revision",
      "/HistoryAwareCommitSummary/properties/historyGeneration",
      "/HistoryAwareCommitSummary/properties/historyCount",
      "/HistoryAwareCommitSummary/properties/historyCursor"
    ],
    "x-requiredSemanticChecks": [
      "historyCount>=1 for loaded document;0<=cursor<count",
      "undoAvailable iff cursor>0; redoAvailable iff cursor+1<count",
      "revision strictly increases on true content commit, nochange keeps revision",
      "preserve-history-cursor does not change historyGeneration/count/cursor/cursorSnapshotDigest/redo, but live digest may change",
      "empty/new-no-document has no commit summary; no fake snapshot digest for cursorless state",
      "internal producer sequence/effect is never accepted as caller authorization"
    ]
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:draft:scientific:structure-geometry:2",
    "title": "SG/2 author-design successor components",
    "description": "Design-only independent namespace; does not mutate SG1-r2,bounded/3 or registered177 operations. Every object closed. No generic rawJSON/harmonicplaceholder. Scalar/frame semantic invariants are normative alongside JSONshape.",
    "oneOf": [
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "phase-placement"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/PhasePlacementSet"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      },
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "group-setting"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/GroupSettingState"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      },
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "adp-render-plan"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/AdpRenderPlan"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      },
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "merge-and-bond-plan"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/MergePlan"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      },
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "plane-fit"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/PlaneFit"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      },
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "native-camera"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/NativeCamera"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      },
      {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "namespace": {
            "const": "vestacraft.scientific.structure-geometry/2"
          },
          "component": {
            "const": "native-dimension-D"
          },
          "ownerId": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "integer",
            "minimum": 0
          },
          "payload": {
            "$ref": "#/$defs/NativeDimensionDState"
          }
        },
        "required": [
          "namespace",
          "component",
          "ownerId",
          "revision",
          "payload"
        ]
      }
    ],
    "$defs": {
      "Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$"
      },
      "SHA256": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "Float32Word": {
        "type": "string",
        "pattern": "^[0-9a-f]{8}$",
        "description": "little-endian4 source bytes,not display endian"
      },
      "Vec3": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Vec3A": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "angstrom"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Vec6A2": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "angstrom^2"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "Matrix3Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Vec3"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Vec4": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "Matrix4Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Vec4"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "Rational": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "numerator": {
            "type": "string",
            "pattern": "^(0|-?[1-9][0-9]*)$"
          },
          "denominator": {
            "type": "string",
            "pattern": "^[1-9][0-9]*$"
          }
        },
        "required": [
          "numerator",
          "denominator"
        ],
        "x-invariants": [
          "gcd(abs(numerator),denominator)=1",
          "canonicalzero=0/1"
        ]
      },
      "Rational3": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Rational"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "RationalMatrix3": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Rational3"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "SourceSpan": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "resourceSHA256": {
            "$ref": "#/$defs/SHA256"
          },
          "byteOffset": {
            "type": "integer",
            "minimum": 0
          },
          "byteLength": {
            "type": "integer",
            "minimum": 1
          },
          "spanSHA256": {
            "$ref": "#/$defs/SHA256"
          }
        },
        "required": [
          "resourceSHA256",
          "byteOffset",
          "byteLength",
          "spanSHA256"
        ]
      },
      "Frame": {
        "oneOf": [
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "periodic-fractional"
              },
              "basisId": {
                "$ref": "#/$defs/Id"
              },
              "basisColumnsInCartesianA": {
                "$ref": "#/$defs/Matrix3Rows"
              }
            },
            "required": [
              "kind",
              "basisId",
              "basisColumnsInCartesianA"
            ],
            "x-invariants": [
              "H columns are lattice vectors;det(H)>0 andnonsingular;notation rows contains mathematicalmatrix rows"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "molecular-cartesian-A"
              },
              "basisId": {
                "$ref": "#/$defs/Id"
              }
            },
            "required": [
              "kind",
              "basisId"
            ]
          }
        ]
      },
      "WorldPose": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "rotationRows": {
            "$ref": "#/$defs/Matrix3Rows"
          },
          "translationA": {
            "$ref": "#/$defs/Vec3A"
          }
        },
        "required": [
          "rotationRows",
          "translationA"
        ],
        "x-invariants": [
          "R^T R=I anddetR=+1 within separatelydeclaredrotationbudget; no implicit orthogonalization",
          "rWorld=R*rLocalCartesianA+tA"
        ]
      },
      "Reference": {
        "oneOf": [
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "internal-cartesian"
              }
            },
            "required": [
              "kind"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "phase"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              }
            },
            "required": [
              "kind",
              "phaseId"
            ]
          }
        ]
      },
      "Anchor": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "kind": {
            "enum": [
              "fractional",
              "cartesian-A"
            ]
          },
          "value": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "kind",
          "value"
        ],
        "x-invariants": [
          "fractional allowed only periodicframe;source anchorunit retained;Cartesian A explicit"
        ]
      },
      "AxisPair": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "firstKind": {
            "enum": [
              "direct",
              "reciprocal"
            ]
          },
          "first": {
            "$ref": "#/$defs/Vec3"
          },
          "second": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "firstKind",
          "first",
          "second"
        ],
        "x-invariants": [
          "secondKind is oppositefirstKind;direct→H*v,reciprocal→inverse(H)^T*h, no2pi",
          "bothnonzero;transformedpair perpendicular within declaredprofile;nativelegacyexceptions distinct"
        ]
      },
      "OrientationConstraint": {
        "oneOf": [
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "resolved-world"
              },
              "rotationRows": {
                "$ref": "#/$defs/Matrix3Rows"
              }
            },
            "required": [
              "kind",
              "rotationRows"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "parallel-axis-pair"
              },
              "reference": {
                "$ref": "#/$defs/Reference"
              },
              "currentAxes": {
                "$ref": "#/$defs/AxisPair"
              },
              "referenceAxes": {
                "$ref": "#/$defs/AxisPair"
              }
            },
            "required": [
              "kind",
              "reference",
              "currentAxes",
              "referenceAxes"
            ]
          }
        ]
      },
      "PositionConstraint": {
        "oneOf": [
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "resolved-world"
              },
              "translationA": {
                "$ref": "#/$defs/Vec3A"
              }
            },
            "required": [
              "kind",
              "translationA"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "anchor-coincidence"
              },
              "reference": {
                "$ref": "#/$defs/Reference"
              },
              "currentAnchor": {
                "$ref": "#/$defs/Anchor"
              },
              "referenceAnchor": {
                "$ref": "#/$defs/Anchor"
              }
            },
            "required": [
              "kind",
              "reference",
              "currentAnchor",
              "referenceAnchor"
            ]
          }
        ]
      },
      "NativeLayerCurrent": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "ltranslSelectorI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "lorientSelectorI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "lorientRawFlagsU32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "currentPositionAnchor6": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "firstOrientationVectors6": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "secondOrientationVectors6": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "lmatrixLogicalRows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "separateTranslationF32A": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -3.4028234663852886e+38,
              "maximum": 3.4028234663852886e+38,
              "x-unit": "angstrom",
              "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "ltranslSelectorI32",
          "lorientSelectorI32",
          "lorientRawFlagsU32",
          "currentPositionAnchor6",
          "firstOrientationVectors6",
          "secondOrientationVectors6",
          "lmatrixLogicalRows",
          "separateTranslationF32A",
          "source"
        ],
        "x-producer-consumer": "03native currentlayerstate -> SG/2explicitconstraints/worldpose bridge;two selectors independent;originalalias/history separate",
        "x-invariants": [
          "retain full16f64 andrawflags evenwhen no properrotationprojection; do not collapse two references intooneparent"
        ]
      },
      "PhasePlacement": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "frame": {
            "$ref": "#/$defs/Frame"
          },
          "orientation": {
            "$ref": "#/$defs/OrientationConstraint"
          },
          "position": {
            "$ref": "#/$defs/PositionConstraint"
          },
          "worldPose": {
            "$ref": "#/$defs/WorldPose"
          },
          "nativeCurrent": {
            "$ref": "#/$defs/NativeLayerCurrent"
          },
          "nativeSourceObservation": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "record": {
                "$ref": "#/$defs/NativeLayerSourceObservation"
              },
              "source": {
                "$ref": "#/$defs/SourceSpan"
              }
            },
            "required": [
              "record",
              "source"
            ]
          }
        },
        "required": [
          "phaseId",
          "frame",
          "orientation",
          "position",
          "worldPose"
        ],
        "x-invariants": [
          "sourceobservation immutableinputaliases/4rawflags separatefromnativeCurrent actualselectors/word andcorrectedconstraints/worldPose",
          "a changedcurrentreference cannotrewriteoriginalsourceobservation;source/current identities/content revisions independentlybound"
        ]
      },
      "PhasePlacementSet": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "evaluationProfile": {
            "enum": [
              "corrected-two-pass-axis-pair",
              "native-ascending-source-order"
            ]
          },
          "phases": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PhasePlacement"
            },
            "minItems": 1
          },
          "nativeSourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          }
        },
        "required": [
          "evaluationProfile",
          "phases",
          "nativeSourceOrder"
        ],
        "x-invariants": [
          "uniquephaseId;allreferencesresolvewithinowner/revision",
          "corrected:orientationDAG first,positionDAG second;unionmaycycle",
          "corrected C=[e1,e2,e1xe2],R_i=R_ref*C_ref*C_i^T;t_i=R_pos*H_pos*q+t_pos−R_i*H_i*p",
          "nativeprofileusespinnedframealgorithm/1e-5/identityfallback;self/forward/outofrangebranches reported separately; do not assert corrected graph equivalence"
        ]
      },
      "SymmetryOp3": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "fractionalRotationRows": {
            "$ref": "#/$defs/RationalMatrix3"
          },
          "fractionalTranslation": {
            "$ref": "#/$defs/Rational3"
          },
          "timeSign": {
            "enum": [
              -1,
              1
            ]
          },
          "basisId": {
            "$ref": "#/$defs/Id"
          }
        },
        "required": [
          "operatorId",
          "fractionalRotationRows",
          "fractionalTranslation",
          "timeSign",
          "basisId"
        ],
        "x-invariants": [
          "fractional affinefPrime=R*f+t",
          "Dcart=H*R*inverse(H);polarvector Dcart*v;axialmagnetic timeSign*det(Dcart)*Dcart*m",
          "exactoperator/group closure andmetriccompatibility separatelyvalidated, no float rational guess"
        ]
      },
      "OrdinarySettingKey": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "kind": {
            "const": "native-ordinary-setting"
          },
          "assetSHA256": {
            "$ref": "#/$defs/SHA256"
          },
          "groupNumber": {
            "type": "integer",
            "minimum": 1,
            "maximum": 230
          },
          "settingOrdinal1": {
            "type": "integer",
            "minimum": 1
          },
          "rawHeaderKey": {
            "type": "string",
            "pattern": "^raw-header:[0-9]+:[0-9]+:[0-9]+:[0-9]+:[0-9]+$"
          },
          "rawHeaderFiveI32": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 5,
            "maxItems": 5
          },
          "nativeName": {
            "type": "string",
            "minLength": 1
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "kind",
          "assetSHA256",
          "groupNumber",
          "settingOrdinal1",
          "rawHeaderKey",
          "rawHeaderFiveI32",
          "nativeName",
          "source"
        ],
        "x-invariants": [
          "header[0]=groupNumber,header[1]=settingOrdinal1 forpinned365rows",
          "setting identityincludesassetSHA andordinal;labelalone notidentity; header[2] kept raw,not inventedorigin"
        ]
      },
      "MagneticSettingKey": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "kind": {
            "const": "native-BNS-setting"
          },
          "assetSHA256": {
            "$ref": "#/$defs/SHA256"
          },
          "bnsMajor": {
            "type": "integer",
            "minimum": 1,
            "maximum": 230
          },
          "bnsMinor": {
            "type": "integer",
            "minimum": 1,
            "maximum": 1651
          },
          "bnsText": {
            "type": "string",
            "pattern": "^[0-9]+[.][0-9]+$"
          },
          "nativeName": {
            "type": "string",
            "minLength": 1
          },
          "rawHeader": {
            "type": "string",
            "minLength": 1
          },
          "ogToBnsRawExpression": {
            "type": "string",
            "minLength": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "kind",
          "assetSHA256",
          "bnsMajor",
          "bnsMinor",
          "bnsText",
          "nativeName",
          "rawHeader",
          "ogToBnsRawExpression",
          "source"
        ],
        "x-invariants": [
          "bnsText must matchmajor/minor;sourceBNS magneticunitcellbasis retained",
          "emptyogToBnsExpression meansactual absentrow,not implicitidentityproof"
        ]
      },
      "GroupSettingState": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "selectedSetting": {
            "oneOf": [
              {
                "$ref": "#/$defs/OrdinarySettingKey"
              },
              {
                "$ref": "#/$defs/MagneticSettingKey"
              }
            ]
          },
          "basisId": {
            "$ref": "#/$defs/Id"
          },
          "listedSourceOperatorExpressions": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            },
            "minItems": 1
          },
          "qualifiedAffineOps": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SymmetryOp3"
            },
            "minItems": 1
          },
          "nativeCurrentCellTransformLogicalRows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "nativeCurrentOperatorMatrices": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Matrix4Rows"
            },
            "minItems": 0
          },
          "wyckoffPolicy": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "native-per-component-SU"
              },
              "baseFractionalTolerance": {
                "const": 0.0002
              },
              "comparison": {
                "const": "wrapped-residual<=max(base,coordinate-SU)"
              }
            },
            "required": [
              "kind",
              "baseFractionalTolerance",
              "comparison"
            ]
          }
        },
        "required": [
          "selectedSetting",
          "basisId",
          "listedSourceOperatorExpressions",
          "qualifiedAffineOps",
          "nativeCurrentCellTransformLogicalRows",
          "nativeCurrentOperatorMatrices",
          "wyckoffPolicy"
        ],
        "x-invariants": [
          "sourceoperators authoritative inlistedfractionalframe;origin encodedintranslations,do not inventITCoriginChoice",
          "nativecurrentop matrices maycarry magnetic/timelegacyrepresentation;conversiontoaffine3requires explicitverifiedparsermapping",
          "current3Dops are qualifiedderived state,not whole sourcehistory;no 3Dflatten ofsuperspace"
        ]
      },
      "NativeProbability99": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "kind": {
            "const": "native-f32-table-99"
          },
          "tableSHA256": {
            "const": "ffe88f1f33846feb44e35909f224dcaab4c3ad8cdc5d62df8386fd3b74187701"
          },
          "percent": {
            "type": "integer",
            "minimum": 1,
            "maximum": 99
          },
          "sourceWordLE": {
            "$ref": "#/$defs/Float32Word"
          },
          "radiusScaleF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          }
        },
        "required": [
          "kind",
          "tableSHA256",
          "percent",
          "sourceWordLE",
          "radiusScaleF32"
        ],
        "x-invariants": [
          "word/value must equalpinnedtable[percent−1]; defaultlegacyselector",
          "do not substitutechi3formulaoruniformdecimalrounding; exactsource99words"
        ]
      },
      "MathProbabilityChi3": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "kind": {
            "const": "explicit-chi3-math-profile"
          },
          "profileId": {
            "const": "MP/1"
          },
          "probability": {
            "type": "number",
            "exclusiveMinimum": 0,
            "exclusiveMaximum": 1
          },
          "radiusScale": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "absoluteErrorBound": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1
          }
        },
        "required": [
          "kind",
          "profileId",
          "probability",
          "radiusScale",
          "absoluteErrorBound"
        ]
      },
      "AdpGeometry": {
        "oneOf": [
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "regular-radius-sphere"
              },
              "radiusA": {
                "type": "number",
                "minimum": 0,
                "maximum": 1.7976931348623157e+308
              }
            },
            "required": [
              "kind",
              "radiusA"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "positive-ellipsoid"
              },
              "axisDirectionsColumns": {
                "$ref": "#/$defs/Matrix3Rows"
              },
              "semiAxesA": {
                "$ref": "#/$defs/Vec3A"
              }
            },
            "required": [
              "kind",
              "axisDirectionsColumns",
              "semiAxesA"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "kind": {
                "const": "non-positive-cuboid"
              },
              "axisDirectionsColumns": {
                "$ref": "#/$defs/Matrix3Rows"
              },
              "halfAxesA": {
                "$ref": "#/$defs/Vec3A"
              },
              "nonPositiveEigenAxes": {
                "type": "array",
                "items": {
                  "type": "boolean"
                },
                "minItems": 3,
                "maxItems": 3
              },
              "unitVertices": {
                "const": [
                  [
                    -1,
                    -1,
                    -1
                  ],
                  [
                    -1,
                    -1,
                    1
                  ],
                  [
                    -1,
                    1,
                    1
                  ],
                  [
                    -1,
                    1,
                    -1
                  ],
                  [
                    1,
                    -1,
                    -1
                  ],
                  [
                    1,
                    -1,
                    1
                  ],
                  [
                    1,
                    1,
                    1
                  ],
                  [
                    1,
                    1,
                    -1
                  ]
                ]
              }
            },
            "required": [
              "kind",
              "axisDirectionsColumns",
              "halfAxesA",
              "nonPositiveEigenAxes",
              "unitVertices"
            ]
          }
        ]
      },
      "AdpRenderPlan": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "currentNativeScalar": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "sourceUorBangstrom^2"
          },
          "nativeScalarKind": {
            "enum": [
              "U",
              "B"
            ]
          },
          "currentNativeTensor6": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "nativeTensorKind": {
            "enum": [
              "Uij",
              "beta"
            ]
          },
          "canonicalCartesianU6A2": {
            "$ref": "#/$defs/Vec6A2"
          },
          "probability": {
            "oneOf": [
              {
                "$ref": "#/$defs/NativeProbability99"
              },
              {
                "$ref": "#/$defs/MathProbabilityChi3"
              }
            ]
          },
          "nativeZero6Policy": {
            "enum": [
              "regular-radius",
              "sqrt-scalar-U-or-B"
            ]
          },
          "sourceExactSixZero": {
            "type": "boolean"
          },
          "eigenvaluesA2": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "angstrom^2"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeMarkerColumn3": {
            "type": "array",
            "items": {
              "enum": [
                -1,
                0
              ]
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeThermalMatrixLogicalRows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "geometry": {
            "$ref": "#/$defs/AdpGeometry"
          },
          "nativeSiteFlagsU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          }
        },
        "required": [
          "siteId",
          "currentNativeScalar",
          "nativeScalarKind",
          "currentNativeTensor6",
          "nativeTensorKind",
          "canonicalCartesianU6A2",
          "probability",
          "nativeZero6Policy",
          "sourceExactSixZero",
          "eigenvaluesA2",
          "nativeMarkerColumn3",
          "nativeThermalMatrixLogicalRows",
          "geometry",
          "nativeSiteFlagsU16"
        ],
        "x-invariants": [
          "sixorder11,22,33,12,13,23; scalarandtensor currentvalues independentpersistent",
          "nonzero tensor axisradius=probabilityScale*sqrt(abs(lambda));lambda<=0 marker−1,positive marker0;anynonpositive→cuboidunit8vertices",
          "all6exactzero takesnativebit25branch beforeeigensolver;never equatezero6fallbackwithzeroprincipalaxis",
          "nativef32/f64constants andoperation order usepinnedFCthermalprofile,not ideal8pi2 replacingroundednativeconstant",
          "geometryaxesfinite nonnegative; axisdirectionseigenframe,not rows accidentallytransposed; scientificscatteringPSDpolicy independentfromlegacydisplay"
        ]
      },
      "MergeSiteInput": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "nativeSpeciesToken": {
            "type": "string",
            "pattern": "^[!-~]{1,2}$"
          },
          "coordinate": {
            "$ref": "#/$defs/Vec3"
          },
          "coordinateSU": {
            "$ref": "#/$defs/Vec3"
          },
          "sourceMetadataEntityId": {
            "$ref": "#/$defs/Id"
          }
        },
        "required": [
          "siteId",
          "nativeSpeciesToken",
          "coordinate",
          "coordinateSU",
          "sourceMetadataEntityId"
        ],
        "x-invariants": [
          "SU finite>=0 for qualifiedscience; source invalidraw preservedoutsidecomputation",
          "coordinateunits determined bydeclaredphaseframe;nativeperiodicusesfractional,molecularCartesianA"
        ]
      },
      "MergeGroupResult": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "representativeSiteId": {
            "$ref": "#/$defs/Id"
          },
          "matchedSiteIds": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 1
          },
          "matchedImageCount": {
            "type": "integer",
            "minimum": 1
          },
          "meanCoordinate": {
            "$ref": "#/$defs/Vec3"
          },
          "legacyArithmeticMeanCoordinateSU": {
            "$ref": "#/$defs/Vec3"
          },
          "retainedMetadataFromSiteId": {
            "$ref": "#/$defs/Id"
          }
        },
        "required": [
          "representativeSiteId",
          "matchedSiteIds",
          "matchedImageCount",
          "meanCoordinate",
          "legacyArithmeticMeanCoordinateSU",
          "retainedMetadataFromSiteId"
        ],
        "x-invariants": [
          "nativegreedy representative isfirstremaininginput;retainedmetadataowner=representative",
          "matchedImageCount countsallmatchedsymmetryimages,not justdistinctsourceIds",
          "SU arithmeticmean islegacyoperation,not statisticalerrorpropagation; occupancy/ADP clonedfromfirst,notsummed",
          "mean maylieoutside[0,1);nativefunctiondoesnotfinalwrap"
        ]
      },
      "PublicBondRule": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "ruleId": {
            "$ref": "#/$defs/Id"
          },
          "searchMode": {
            "enum": [
              "specified-pair",
              "any-neighbor-of-first",
              "all-pairs"
            ]
          },
          "selectorKind": {
            "enum": [
              "native-species-token",
              "source-site-label"
            ]
          },
          "selectorA": {
            "type": "string",
            "minLength": 1
          },
          "selectorB": {
            "type": "string",
            "minLength": 1
          },
          "minimumA": {
            "type": "number",
            "minimum": 0,
            "maximum": 3.4028234663852886e+38
          },
          "maximumA": {
            "type": "number",
            "minimum": 0,
            "maximum": 3.4028234663852886e+38
          },
          "boundaryMode": {
            "enum": [
              "none",
              "first-inside-extension",
              "recursive-either-visible"
            ]
          },
          "showPolyhedra": {
            "type": "boolean"
          },
          "nativeRuleSourceId": {
            "$ref": "#/$defs/Id"
          }
        },
        "required": [
          "ruleId",
          "searchMode",
          "selectorKind",
          "selectorA",
          "selectorB",
          "minimumA",
          "maximumA",
          "boundaryMode",
          "showPolyhedra",
          "nativeRuleSourceId"
        ],
        "x-invariants": [
          "minimum<=maximum;legacythresholdsconverted/squaredf32thenpromotedasFCselectedproof",
          "effectiveA/B:pairusesboth,any-neighborusesA,all-pairsusesnone;preserveunusedsourcevaluesindependently",
          "all-pairs cannotbeencodedasXXelement orassumedN^2pairlowering;explicittypedmode",
          "nativeallmodes computationalparity qualification distinctfromcompletecurrentSBOND15sourcecapture"
        ]
      },
      "MergePlan": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "frame": {
            "$ref": "#/$defs/Frame"
          },
          "profile": {
            "enum": [
              "native-greedy-source-order",
              "detect-connected-components",
              "corrected-metric-greedy"
            ]
          },
          "thresholdA": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 3.4028234663852886e+38
          },
          "sourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 1
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MergeSiteInput"
            },
            "minItems": 1
          },
          "symmetryOps": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SymmetryOp3"
            },
            "minItems": 1
          },
          "groups": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MergeGroupResult"
            },
            "minItems": 0
          },
          "bondRules": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PublicBondRule"
            },
            "minItems": 0
          }
        },
        "required": [
          "phaseId",
          "frame",
          "profile",
          "thresholdA",
          "sourceOrder",
          "sites",
          "symmetryOps",
          "groups",
          "bondRules"
        ],
        "x-invariants": [
          "nativeprofile strictdistance<threshold;sourceorderfixed;speciesstrcmp",
          "nativeperiodic image roundingcomponentwise floor(delta+.5),notmetricclosestimage;correctedmetricprofileexplicitseparate",
          "nativegreedy fixedfirstanchorstar grouping;no transitiveclosure;detectprofilecannotmutate merge",
          "allidsunique/resolvedowner;exact f32 thresholdprojection plusdeclarednumericalprofile",
          "threshold<=0/emptyzero-match/nonfinite returntypedfailure incommandadapter,notdivide0success"
        ]
      },
      "PlaneFitInput": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "profile": {
            "enum": [
              "native-homogeneous-TLS",
              "corrected-centered-orthogonal-LS"
            ]
          },
          "pointsCartesianA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3A"
            },
            "minItems": 0
          },
          "inputNormalForNativeSmallN": {
            "$ref": "#/$defs/Vec3"
          },
          "sourceSelectionIds": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          }
        },
        "required": [
          "profile",
          "pointsCartesianA",
          "inputNormalForNativeSmallN",
          "sourceSelectionIds"
        ],
        "x-invariants": [
          "samecountpoints/selectionIds;sourceorderpreserved",
          "nativeN1preservesinputnormal;N2usesauxiliarysourcebranch;N>=3SVDrows[x,y,z,−1]",
          "corrected requiresnoncollinearpointsetforuniqueplane;centered3DSVD,unitnormal,firstnonzeroCartesiancomponentpositive"
        ]
      },
      "PlaneFitResolved": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "status": {
            "const": "resolved"
          },
          "sourceReturnCode": {
            "type": "integer"
          },
          "pointCount": {
            "type": "integer",
            "minimum": 1
          },
          "sourceHomogeneous4": {
            "$ref": "#/$defs/Vec4"
          },
          "unitNormalCartesian": {
            "$ref": "#/$defs/Vec3"
          },
          "offsetA": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "angstrom"
          },
          "signPolicy": {
            "enum": [
              "source-SVD-sign-not-canonical",
              "first-nonzero-component-positive"
            ]
          }
        },
        "required": [
          "status",
          "sourceReturnCode",
          "pointCount",
          "sourceHomogeneous4",
          "unitNormalCartesian",
          "offsetA",
          "signPolicy"
        ],
        "x-invariants": [
          "plane nUnit dot rA=offsetA;source4=[nx,ny,nz,d]not arbitraryhomogeneoussignassumption",
          "nativeTLS normalizes4numericcoefficients;physicalnormalconversiondividesby||n||"
        ]
      },
      "PlaneFitUnderdetermined": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "status": {
            "enum": [
              "empty-selection",
              "underdetermined"
            ]
          },
          "sourceReturnCode": {
            "type": "integer"
          },
          "pointCount": {
            "type": "integer",
            "minimum": 0
          },
          "affineRank": {
            "type": "integer",
            "minimum": 0,
            "maximum": 2
          },
          "reason": {
            "enum": [
              "no-points",
              "coincident-points",
              "collinear-points",
              "zero-input-normal",
              "native-third-singular-below-1e-6"
            ]
          }
        },
        "required": [
          "status",
          "sourceReturnCode",
          "pointCount",
          "affineRank",
          "reason"
        ]
      },
      "PlaneFit": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "input": {
            "$ref": "#/$defs/PlaneFitInput"
          },
          "result": {
            "oneOf": [
              {
                "$ref": "#/$defs/PlaneFitResolved"
              },
              {
                "$ref": "#/$defs/PlaneFitUnderdetermined"
              }
            ]
          }
        },
        "required": [
          "input",
          "result"
        ],
        "x-invariants": [
          "nativeabs(thirdsingularvalue)<1e−6 withN>=3returns2; thresholdisnativeprofileonly",
          "futureactualSVDnumericqualificationisnot a missingpayloadfield"
        ]
      },
      "NativeCamera": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "sceneMatrixLogicalRows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "shiftViewX": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "view-coordinate",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "shiftViewY": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "view-coordinate",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "thirdSceneStoredF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "source-unassigned-preserved",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "zoomF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "projectionSelectorBit21": {
            "enum": [
              0,
              1
            ]
          },
          "projectionZScaleF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "localTranslationA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -3.4028234663852886e+38,
              "maximum": 3.4028234663852886e+38,
              "x-unit": "angstrom",
              "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "normalizationF64": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "1/angstrom"
          },
          "viewportLogicalWidth": {
            "type": "integer",
            "minimum": 1
          },
          "viewportLogicalHeight": {
            "type": "integer",
            "minimum": 1
          },
          "deviceScaleF64": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "aspectF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "aspectScaleF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "projectionContext": {
            "enum": [
              "capture-restored-normal",
              "tiled-capture",
              "explicit-canonical-camera"
            ]
          },
          "nearView": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "farView": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          }
        },
        "required": [
          "sceneMatrixLogicalRows",
          "shiftViewX",
          "shiftViewY",
          "thirdSceneStoredF32",
          "zoomF32",
          "projectionSelectorBit21",
          "projectionZScaleF32",
          "localTranslationA",
          "normalizationF64",
          "viewportLogicalWidth",
          "viewportLogicalHeight",
          "deviceScaleF64",
          "aspectF32",
          "aspectScaleF32",
          "projectionContext",
          "nearView",
          "farView"
        ],
        "x-invariants": [
          "source Mview=T(0,0,-5)*T(shiftX,shiftY,0)*Sxy,z*[Sz(PROJTscalar)ifbit21]*SceneM*Tlocal",
          "sxy=f32(f64(zoom)*normalization*f64(aspectScale));sz=normalization*f64(f32(min(zoom,4)*aspectScale))",
          "aspect=f32(width/height);aspectScale=1 ifwidth<=height elsef32(height/width); physicalviewporttrunc(logical*deviceScale)",
          "full16f64affine source preserved;not forcedintoquaternion;shiftView notpanWorldA",
          "PROJTscalar ismodelviewzscale,notFOV;near<far;context-specificsourceprojection separatelyqualified",
          "capturesourcebit0 ortho±1,y±1/aspect;bit1 frustum±.2,y±.2/aspect,near1far10 atrestore;tile contexts source-specific",
          "bounds-derivedlocaltranslation/normalizationinputs tiedsamegeometryrevision,not stale sourcecamera history"
        ]
      },
      "FC1_Entry": {
        "type": "object",
        "properties": {
          "kind": {
            "enum": [
              "phase",
              "molecule",
              "source-entity"
            ]
          },
          "id": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "kind",
          "id"
        ],
        "additionalProperties": false
      },
      "FC1_SourceScalar": {
        "type": "object",
        "properties": {
          "literal": {
            "type": "string",
            "minLength": 1
          },
          "nativeF64Bits": {
            "type": "string",
            "pattern": "^[0-9a-f]{16}$"
          }
        },
        "required": [
          "literal",
          "nativeF64Bits"
        ],
        "additionalProperties": false
      },
      "FC1_Reference": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "internal-cartesian"
              }
            },
            "required": [
              "kind"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "entry"
              },
              "entry": {
                "$ref": "#/$defs/FC1_Entry"
              }
            },
            "required": [
              "kind",
              "entry"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "unresolved"
              },
              "rawSelector": {
                "type": "integer",
                "minimum": -2147483648,
                "maximum": 2147483647
              }
            },
            "required": [
              "kind",
              "rawSelector"
            ],
            "additionalProperties": false
          }
        ]
      },
      "NativeLayerSourceObservation": {
        "type": "object",
        "properties": {
          "schemaVersion": {
            "const": "NativeLayerSource-proposal-1"
          },
          "owner": {
            "$ref": "#/$defs/FC1_Entry"
          },
          "position": {
            "type": "object",
            "properties": {
              "rawSelector": {
                "type": "integer",
                "minimum": -2147483648,
                "maximum": 2147483647
              },
              "reference": {
                "$ref": "#/$defs/FC1_Reference"
              },
              "currentAnchor": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/FC1_SourceScalar"
                },
                "minItems": 3,
                "maxItems": 3
              },
              "referenceAnchor": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/FC1_SourceScalar"
                },
                "minItems": 3,
                "maxItems": 3
              }
            },
            "required": [
              "rawSelector",
              "reference",
              "currentAnchor",
              "referenceAnchor"
            ],
            "additionalProperties": false
          },
          "orientation": {
            "type": "object",
            "properties": {
              "rawSelector": {
                "type": "integer",
                "minimum": -2147483648,
                "maximum": 2147483647
              },
              "reference": {
                "$ref": "#/$defs/FC1_Reference"
              },
              "currentFirst": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/FC1_SourceScalar"
                },
                "minItems": 3,
                "maxItems": 3
              },
              "referenceFirst": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/FC1_SourceScalar"
                },
                "minItems": 3,
                "maxItems": 3
              },
              "currentSecond": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/FC1_SourceScalar"
                },
                "minItems": 3,
                "maxItems": 3
              },
              "referenceSecond": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/FC1_SourceScalar"
                },
                "minItems": 3,
                "maxItems": 3
              },
              "rawFlags": {
                "type": "array",
                "items": {
                  "type": "integer",
                  "minimum": -2147483648,
                  "maximum": 2147483647
                },
                "minItems": 4,
                "maxItems": 4
              }
            },
            "required": [
              "rawSelector",
              "reference",
              "currentFirst",
              "referenceFirst",
              "currentSecond",
              "referenceSecond",
              "rawFlags"
            ],
            "additionalProperties": false
          },
          "sourceProjection": {
            "const": "not-established-by-record-alone"
          }
        },
        "required": [
          "schemaVersion",
          "owner",
          "position",
          "orientation",
          "sourceProjection"
        ],
        "additionalProperties": false,
        "title": "Typed source metadata before closed AuditNode encoding; not an SG/protocol admission",
        "x-sealed-source": {
          "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-layer-source.schema.json",
          "sha256": "db4cf31ca3efe9c39f80ab973e23875b97e04e603c15afeec154702ad051d41e",
          "importMode": "exactshapeinline,localrefsrenamedFC1_"
        }
      },
      "Vec6": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "NativeOccupationDomainDefinition": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "domainId": {
            "$ref": "#/$defs/Id"
          },
          "sourceShapeOrdinal": {
            "type": "integer",
            "minimum": 0
          },
          "nativeCoordinateWidth": {
            "const": 6
          },
          "verticesNative6": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec6"
            },
            "minItems": 0
          },
          "nativeNthIndices0based": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            },
            "minItems": 0
          },
          "nativeVertexCount": {
            "type": "integer",
            "minimum": 0
          },
          "nativeNthCount": {
            "type": "integer",
            "minimum": 0
          },
          "externalComponentNames": {
            "const": [
              "xe1",
              "xe2",
              "xe3"
            ]
          },
          "externalDisplacementLengthsA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "angstrom"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "externalDirectionRowsNative6": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec6"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "internalShiftDirectionNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "internalShiftScale": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "native-internal-scale"
          },
          "currentDerivedTranslationNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "externalComponentSU": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "angstrom"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "externalComponentStep": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "angstrom"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeParameterFlagsI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          },
          "nativeSymmetryOperatorOrdinal0based": {
            "type": "integer",
            "minimum": 0
          },
          "nativeDomainScale": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "native-source-domain-scalar"
          },
          "currentMaximumInternalRadiusSquared": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "native-derived-domain-scalar"
          }
        },
        "required": [
          "domainId",
          "sourceShapeOrdinal",
          "nativeCoordinateWidth",
          "verticesNative6",
          "nativeNthIndices0based",
          "nativeVertexCount",
          "nativeNthCount",
          "externalComponentNames",
          "externalDisplacementLengthsA",
          "externalDirectionRowsNative6",
          "internalShiftDirectionNative6",
          "internalShiftScale",
          "nativeDomainScale",
          "currentMaximumInternalRadiusSquared",
          "currentDerivedTranslationNative6",
          "externalComponentSU",
          "externalComponentStep",
          "nativeParameterFlagsI32",
          "source",
          "nativeSymmetryOperatorOrdinal0based"
        ],
        "x-producer-consumer": "readdata40/41 XQcODfrac @9b754..9bf2c -> site+138 -> BuildQuasiCrystal @18bc8..191xx -> DrawODsImpl; actualnativephaseoffsets and per-domain typedgeometry",
        "x-invariants": [
          "nativeVertexCount=len(verticesNative6),nativeNthCount=len(nativeNthIndices0based)",
          "fixed6 is sourceXQcODcoordinatewidth,not invented6harmonicaxes;D5read-path performs explicit6to5 conversion andretains sourceoriginalindependently",
          "for each xe_j>1e-6: v=C2O*direction_j;normalizefirst3Cartesiancomponents tolengthxe_j;clearremainingD-3;O2C*v addsderiveddomaintranslation",
          "commonshift: C2O*(internalShiftScale*direction);clearfirst3physicalcomponents;O2C mapsremainingbacktocurrentDcoordinates",
          "source+18vertices6,+70nthI32,+50ordinal,+a0/a4counts,+b0values,+c8rows,+158direction,+188scale,+190derived,+1c8SU,+1e0step,+1f8flags;no genericJSON",
          "basecontainers+38/+58/+88 derivefromsourcevertices/nth via nativebuild profile;rebuild caches,donotinventmissingoriginalvalues",
          "nativeSymmetryOperatorOrdinal0based=+54,sourceShapeOrdinal=+50:BuildQuasiCrystal191d0..191e4 selects UnitCell+70 operator andCrystal+1f8 template separately",
          "nativeDomainScale=+a8:192c4 alpha*C2O;19334 alpha=1/nativeDomainScale*O2C;source zero causes native divisioninvalid;correctedplan diagnostics E_DEGENERATE_GEOMETRY instead ofinventing scale",
          "currentMaximumInternalRadiusSquared=+1c0: max overdomainvertices of sum(physical-transformedinternalcomponents[3:D]^2),afterzeroingphysical3;exactnativecache notADP",
          "nth D5 pairs index2internalvertices andmeasure|det2|/2 @19434..19474;D6 triples measure|det3| @1949c..19504 (native no/6);not rounded operationindices",
          "per-site verticesNative6 andtemplatevertices separateactualstate: R_selected*template,then nativeDomainScale*C2O,physicalfirst3zero,then1/nativeDomainScale*O2C rebuildcurrentdomaininternals",
          "actualsourcepolygon/tetrahedron ordering retained;sourcefunction profile fixesdrawtriangulation,not unorderedpointcloud"
        ]
      },
      "VectorD3": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "MatrixD3Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/VectorD3"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "MatrixD4HomogeneousRows": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "minItems": 4,
        "maxItems": 4
      },
      "NativeSiteD3": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "nativeSpeciesToken": {
            "type": "string",
            "pattern": "^[!-~]{1,2}$"
          },
          "nativeElementIndexI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          },
          "fractionalCoordinatesD": {
            "$ref": "#/$defs/VectorD3"
          },
          "occupancySlotD": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "nativeScalarSlotD1": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "sourceUorBangstrom^2"
          },
          "thermal6SlotsD2ToD7": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "coordinateSUD": {
            "$ref": "#/$defs/VectorD3"
          },
          "nativeSiteArrayBlocks3ByDPlus8": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 11,
              "maxItems": 11
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeFlagsU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "occupationDomainId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "nativeOrbitNumeratorI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          }
        },
        "required": [
          "siteId",
          "nativeSpeciesToken",
          "nativeElementIndexI16",
          "fractionalCoordinatesD",
          "occupancySlotD",
          "nativeScalarSlotD1",
          "thermal6SlotsD2ToD7",
          "coordinateSUD",
          "nativeSiteArrayBlocks3ByDPlus8",
          "nativeFlagsU16",
          "occupationDomainId",
          "nativeOrbitNumeratorI16"
        ],
        "x-invariants": [
          "XSite+18=3;allocated3*(3+8)f64 @c6f50..c6f80",
          "explicitcoordinates/scalar/tensor/SU mustequalactualblockprojections;block0[:D]=coordinates,block0[D]=occupancy,block0[D+1]=scalar,block0[D+2:D+8]=thermal6,block1[:D]=coordinateSU",
          "nativeFlagsU16 isactualXSite+da,bit3 usedbyactualscatter path;not replaced byglobal THERT/THERI/table flags",
          "block2 andunconsumedblockfields exacttypedcurrentdata,not imaginaryharmoniccoefficients; originalsourceobservationindependent",
          "nativeOrbitNumeratorI16 is signedXSite+d8 used byactualSF/densityweight;do notdecodeunsigned merelybecause storage16bits"
        ]
      },
      "NativeSymmetryOpD3": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "currentNativeMatrixRows": {
            "$ref": "#/$defs/MatrixD4HomogeneousRows"
          },
          "timeSign": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "operatorId",
          "currentNativeMatrixRows",
          "timeSign"
        ],
        "x-invariants": [
          "positionaffine usesfirstDrows/cols pluscolDtranslation;nativebottomrowandtime representation keptactual,not rewrittenasordinaryhomogeneous1",
          "currentUnitCell+70 fullDoplist,not3Dtruncate ofgenerator/sourcehistory"
        ]
      },
      "NativeDimensionD3State": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "modelId": {
            "$ref": "#/$defs/Id"
          },
          "dimensionD": {
            "const": 3
          },
          "nativeMatrixDimensionWord": {
            "const": 4
          },
          "nativeModelKind": {
            "enum": [
              "crystal",
              "molecule",
              "quasicrystal",
              "magnetic-quasicrystal"
            ]
          },
          "sourceGroupNumberI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "sourceSettingOrdinalI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "crystallographicToOrthoRowsD": {
            "$ref": "#/$defs/MatrixD3Rows"
          },
          "orthoToCrystallographicRowsD": {
            "$ref": "#/$defs/MatrixD3Rows"
          },
          "phasonRowsD": {
            "$ref": "#/$defs/MatrixD3Rows"
          },
          "layerWorldMatrix4Rows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "currentC2XYZRowsDp1": {
            "$ref": "#/$defs/MatrixD4HomogeneousRows"
          },
          "currentXYZ2CRowsDp1": {
            "$ref": "#/$defs/MatrixD4HomogeneousRows"
          },
          "nativeAuxInverse1A0RowsDp1": {
            "$ref": "#/$defs/MatrixD4HomogeneousRows"
          },
          "qcorigNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "nativeCIFWaveVectorRows3": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "drawingBoundsMinMaxD": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -3.4028234663852886e+38,
                "maximum": 3.4028234663852886e+38,
                "x-unit": "native-fractional-bound",
                "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
              },
              "minItems": 2,
              "maxItems": 2
            },
            "minItems": 3,
            "maxItems": 3
          },
          "symmetryOpsD": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSymmetryOpD3"
            },
            "minItems": 1
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSiteD3"
            },
            "minItems": 0
          },
          "occupationDomains": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainDefinition"
            },
            "minItems": 0
          },
          "nativeSourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          },
          "occupationDomainTemplates": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainTemplate"
            },
            "minItems": 0
          }
        },
        "required": [
          "modelId",
          "dimensionD",
          "nativeMatrixDimensionWord",
          "nativeModelKind",
          "sourceGroupNumberI32",
          "sourceSettingOrdinalI32",
          "crystallographicToOrthoRowsD",
          "orthoToCrystallographicRowsD",
          "phasonRowsD",
          "layerWorldMatrix4Rows",
          "currentC2XYZRowsDp1",
          "currentXYZ2CRowsDp1",
          "nativeAuxInverse1A0RowsDp1",
          "qcorigNative6",
          "nativeCIFWaveVectorRows3",
          "drawingBoundsMinMaxD",
          "symmetryOpsD",
          "sites",
          "occupationDomains",
          "nativeSourceOrder",
          "source",
          "occupationDomainTemplates"
        ],
        "x-producer-consumer": "03native-source/document actualcurrentD capture -> Scene::BuildOrientMatrix -> BuildCrystal2(D3)/BuildCluster(D4)/BuildQuasiCrystal(D5/6) -> physical display/scatter/currenttypedarchive",
        "x-invariants": [
          "dimensionD isactualmathematicalD;Crystal+168 storesD+1,notD;nativeQUASICRYSTAL DIM=constructor receivesD",
          "C2O/O2C fromGetMatrixC2O/+510 andGetMatrixO2C/+538;fullDmatrix retained;PHASON at+588;QCORIG at+78..a0 sixsourcef64",
          "GetMatrixC2XYZ+150/GetMatrixXYZ2C+178 returnfull(D+1)^2;notonly16values forD>3",
          "BuildOrientMatrix: temp=PHASON*C2O;Initialize copiesmin dimensions intoexistingC2XYZ withoutresize;GeMM(M4,temp)updatesM=min(D+1,4),N=D,K=min(4,D),otherrows/edgevaluesretainactualpreviousstate;translationcolD first3 updatedbyindependentpositionreference rule",
          "InitializeI buildsXYZ2C inverse;nativeAux1A0 copiesinverse andnegateslastcolumnfirst3;fullsourceedgewords retained,not implicitzero padding",
          "higherD positions usecurrentDops andcompleteDcoordinates;projectphysicalfirst3onlyafterD-affine evaluation;internalD-3 componentsremainmodel/identity",
          "nativeCIFwaveq rows storedordered3;do notassignunprovedharmonicfunctionmeaning;actualhigherDstructurefactors useDdotproduct perFS evidence",
          "allsite/domain/opids resolvewithinsameowner/revision;actualDcoordinates survivetypedarchive/change/restore; no source-onlydowngrade",
          "nativefileprojection/save usesactualqualifiedgrammar/full loss report;internalenginearchivepersists alltypedDfields andsource independently;never stripsinternalaxes",
          "scene builddispatchnativeword4→D3BuildCrystal2,word5→D4BuildCluster,elseD5/6BuildQuasiCrystal;runtimequalificationnot virtual exclusion",
          "templateOrdinal resolves Crystal+1f8 sourcegeometry independentlyof per-sitecopy;siteOD currentvertices mayalreadytransformed;never usemutatedcurrentcopy assource template"
        ]
      },
      "VectorD4": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "MatrixD4Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/VectorD4"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "MatrixD5HomogeneousRows": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "minItems": 5,
          "maxItems": 5
        },
        "minItems": 5,
        "maxItems": 5
      },
      "NativeSiteD4": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "nativeSpeciesToken": {
            "type": "string",
            "pattern": "^[!-~]{1,2}$"
          },
          "nativeElementIndexI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          },
          "fractionalCoordinatesD": {
            "$ref": "#/$defs/VectorD4"
          },
          "occupancySlotD": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "nativeScalarSlotD1": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "sourceUorBangstrom^2"
          },
          "thermal6SlotsD2ToD7": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "coordinateSUD": {
            "$ref": "#/$defs/VectorD4"
          },
          "nativeSiteArrayBlocks3ByDPlus8": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 12,
              "maxItems": 12
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeFlagsU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "occupationDomainId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "nativeOrbitNumeratorI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          }
        },
        "required": [
          "siteId",
          "nativeSpeciesToken",
          "nativeElementIndexI16",
          "fractionalCoordinatesD",
          "occupancySlotD",
          "nativeScalarSlotD1",
          "thermal6SlotsD2ToD7",
          "coordinateSUD",
          "nativeSiteArrayBlocks3ByDPlus8",
          "nativeFlagsU16",
          "occupationDomainId",
          "nativeOrbitNumeratorI16"
        ],
        "x-invariants": [
          "XSite+18=4;allocated3*(4+8)f64 @c6f50..c6f80",
          "explicitcoordinates/scalar/tensor/SU mustequalactualblockprojections;block0[:D]=coordinates,block0[D]=occupancy,block0[D+1]=scalar,block0[D+2:D+8]=thermal6,block1[:D]=coordinateSU",
          "nativeFlagsU16 isactualXSite+da,bit3 usedbyactualscatter path;not replaced byglobal THERT/THERI/table flags",
          "block2 andunconsumedblockfields exacttypedcurrentdata,not imaginaryharmoniccoefficients; originalsourceobservationindependent",
          "nativeOrbitNumeratorI16 is signedXSite+d8 used byactualSF/densityweight;do notdecodeunsigned merelybecause storage16bits"
        ]
      },
      "NativeSymmetryOpD4": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "currentNativeMatrixRows": {
            "$ref": "#/$defs/MatrixD5HomogeneousRows"
          },
          "timeSign": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "operatorId",
          "currentNativeMatrixRows",
          "timeSign"
        ],
        "x-invariants": [
          "positionaffine usesfirstDrows/cols pluscolDtranslation;nativebottomrowandtime representation keptactual,not rewrittenasordinaryhomogeneous1",
          "currentUnitCell+70 fullDoplist,not3Dtruncate ofgenerator/sourcehistory"
        ]
      },
      "NativeDimensionD4State": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "modelId": {
            "$ref": "#/$defs/Id"
          },
          "dimensionD": {
            "const": 4
          },
          "nativeMatrixDimensionWord": {
            "const": 5
          },
          "nativeModelKind": {
            "enum": [
              "crystal",
              "molecule",
              "quasicrystal",
              "magnetic-quasicrystal"
            ]
          },
          "sourceGroupNumberI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "sourceSettingOrdinalI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "crystallographicToOrthoRowsD": {
            "$ref": "#/$defs/MatrixD4Rows"
          },
          "orthoToCrystallographicRowsD": {
            "$ref": "#/$defs/MatrixD4Rows"
          },
          "phasonRowsD": {
            "$ref": "#/$defs/MatrixD4Rows"
          },
          "layerWorldMatrix4Rows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "currentC2XYZRowsDp1": {
            "$ref": "#/$defs/MatrixD5HomogeneousRows"
          },
          "currentXYZ2CRowsDp1": {
            "$ref": "#/$defs/MatrixD5HomogeneousRows"
          },
          "nativeAuxInverse1A0RowsDp1": {
            "$ref": "#/$defs/MatrixD5HomogeneousRows"
          },
          "qcorigNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "nativeCIFWaveVectorRows3": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "drawingBoundsMinMaxD": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -3.4028234663852886e+38,
                "maximum": 3.4028234663852886e+38,
                "x-unit": "native-fractional-bound",
                "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
              },
              "minItems": 2,
              "maxItems": 2
            },
            "minItems": 4,
            "maxItems": 4
          },
          "symmetryOpsD": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSymmetryOpD4"
            },
            "minItems": 1
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSiteD4"
            },
            "minItems": 0
          },
          "occupationDomains": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainDefinition"
            },
            "minItems": 0
          },
          "nativeSourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          },
          "occupationDomainTemplates": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainTemplate"
            },
            "minItems": 0
          }
        },
        "required": [
          "modelId",
          "dimensionD",
          "nativeMatrixDimensionWord",
          "nativeModelKind",
          "sourceGroupNumberI32",
          "sourceSettingOrdinalI32",
          "crystallographicToOrthoRowsD",
          "orthoToCrystallographicRowsD",
          "phasonRowsD",
          "layerWorldMatrix4Rows",
          "currentC2XYZRowsDp1",
          "currentXYZ2CRowsDp1",
          "nativeAuxInverse1A0RowsDp1",
          "qcorigNative6",
          "nativeCIFWaveVectorRows3",
          "drawingBoundsMinMaxD",
          "symmetryOpsD",
          "sites",
          "occupationDomains",
          "nativeSourceOrder",
          "source",
          "occupationDomainTemplates"
        ],
        "x-producer-consumer": "03native-source/document actualcurrentD capture -> Scene::BuildOrientMatrix -> BuildCrystal2(D3)/BuildCluster(D4)/BuildQuasiCrystal(D5/6) -> physical display/scatter/currenttypedarchive",
        "x-invariants": [
          "dimensionD isactualmathematicalD;Crystal+168 storesD+1,notD;nativeQUASICRYSTAL DIM=constructor receivesD",
          "C2O/O2C fromGetMatrixC2O/+510 andGetMatrixO2C/+538;fullDmatrix retained;PHASON at+588;QCORIG at+78..a0 sixsourcef64",
          "GetMatrixC2XYZ+150/GetMatrixXYZ2C+178 returnfull(D+1)^2;notonly16values forD>3",
          "BuildOrientMatrix: temp=PHASON*C2O;Initialize copiesmin dimensions intoexistingC2XYZ withoutresize;GeMM(M4,temp)updatesM=min(D+1,4),N=D,K=min(4,D),otherrows/edgevaluesretainactualpreviousstate;translationcolD first3 updatedbyindependentpositionreference rule",
          "InitializeI buildsXYZ2C inverse;nativeAux1A0 copiesinverse andnegateslastcolumnfirst3;fullsourceedgewords retained,not implicitzero padding",
          "higherD positions usecurrentDops andcompleteDcoordinates;projectphysicalfirst3onlyafterD-affine evaluation;internalD-3 componentsremainmodel/identity",
          "nativeCIFwaveq rows storedordered3;do notassignunprovedharmonicfunctionmeaning;actualhigherDstructurefactors useDdotproduct perFS evidence",
          "allsite/domain/opids resolvewithinsameowner/revision;actualDcoordinates survivetypedarchive/change/restore; no source-onlydowngrade",
          "nativefileprojection/save usesactualqualifiedgrammar/full loss report;internalenginearchivepersists alltypedDfields andsource independently;never stripsinternalaxes",
          "scene builddispatchnativeword4→D3BuildCrystal2,word5→D4BuildCluster,elseD5/6BuildQuasiCrystal;runtimequalificationnot virtual exclusion",
          "templateOrdinal resolves Crystal+1f8 sourcegeometry independentlyof per-sitecopy;siteOD currentvertices mayalreadytransformed;never usemutatedcurrentcopy assource template"
        ]
      },
      "VectorD5": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 5,
        "maxItems": 5
      },
      "MatrixD5Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/VectorD5"
        },
        "minItems": 5,
        "maxItems": 5
      },
      "MatrixD6HomogeneousRows": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "minItems": 6,
          "maxItems": 6
        },
        "minItems": 6,
        "maxItems": 6
      },
      "NativeSiteD5": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "nativeSpeciesToken": {
            "type": "string",
            "pattern": "^[!-~]{1,2}$"
          },
          "nativeElementIndexI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          },
          "fractionalCoordinatesD": {
            "$ref": "#/$defs/VectorD5"
          },
          "occupancySlotD": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "nativeScalarSlotD1": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "sourceUorBangstrom^2"
          },
          "thermal6SlotsD2ToD7": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "coordinateSUD": {
            "$ref": "#/$defs/VectorD5"
          },
          "nativeSiteArrayBlocks3ByDPlus8": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 13,
              "maxItems": 13
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeFlagsU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "occupationDomainId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "nativeOrbitNumeratorI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          }
        },
        "required": [
          "siteId",
          "nativeSpeciesToken",
          "nativeElementIndexI16",
          "fractionalCoordinatesD",
          "occupancySlotD",
          "nativeScalarSlotD1",
          "thermal6SlotsD2ToD7",
          "coordinateSUD",
          "nativeSiteArrayBlocks3ByDPlus8",
          "nativeFlagsU16",
          "occupationDomainId",
          "nativeOrbitNumeratorI16"
        ],
        "x-invariants": [
          "XSite+18=5;allocated3*(5+8)f64 @c6f50..c6f80",
          "explicitcoordinates/scalar/tensor/SU mustequalactualblockprojections;block0[:D]=coordinates,block0[D]=occupancy,block0[D+1]=scalar,block0[D+2:D+8]=thermal6,block1[:D]=coordinateSU",
          "nativeFlagsU16 isactualXSite+da,bit3 usedbyactualscatter path;not replaced byglobal THERT/THERI/table flags",
          "block2 andunconsumedblockfields exacttypedcurrentdata,not imaginaryharmoniccoefficients; originalsourceobservationindependent",
          "nativeOrbitNumeratorI16 is signedXSite+d8 used byactualSF/densityweight;do notdecodeunsigned merelybecause storage16bits"
        ]
      },
      "NativeSymmetryOpD5": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "currentNativeMatrixRows": {
            "$ref": "#/$defs/MatrixD6HomogeneousRows"
          },
          "timeSign": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "operatorId",
          "currentNativeMatrixRows",
          "timeSign"
        ],
        "x-invariants": [
          "positionaffine usesfirstDrows/cols pluscolDtranslation;nativebottomrowandtime representation keptactual,not rewrittenasordinaryhomogeneous1",
          "currentUnitCell+70 fullDoplist,not3Dtruncate ofgenerator/sourcehistory"
        ]
      },
      "NativeDimensionD5State": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "modelId": {
            "$ref": "#/$defs/Id"
          },
          "dimensionD": {
            "const": 5
          },
          "nativeMatrixDimensionWord": {
            "const": 6
          },
          "nativeModelKind": {
            "enum": [
              "crystal",
              "molecule",
              "quasicrystal",
              "magnetic-quasicrystal"
            ]
          },
          "sourceGroupNumberI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "sourceSettingOrdinalI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "crystallographicToOrthoRowsD": {
            "$ref": "#/$defs/MatrixD5Rows"
          },
          "orthoToCrystallographicRowsD": {
            "$ref": "#/$defs/MatrixD5Rows"
          },
          "phasonRowsD": {
            "$ref": "#/$defs/MatrixD5Rows"
          },
          "layerWorldMatrix4Rows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "currentC2XYZRowsDp1": {
            "$ref": "#/$defs/MatrixD6HomogeneousRows"
          },
          "currentXYZ2CRowsDp1": {
            "$ref": "#/$defs/MatrixD6HomogeneousRows"
          },
          "nativeAuxInverse1A0RowsDp1": {
            "$ref": "#/$defs/MatrixD6HomogeneousRows"
          },
          "qcorigNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "nativeCIFWaveVectorRows3": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "drawingBoundsMinMaxD": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -3.4028234663852886e+38,
                "maximum": 3.4028234663852886e+38,
                "x-unit": "native-fractional-bound",
                "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
              },
              "minItems": 2,
              "maxItems": 2
            },
            "minItems": 5,
            "maxItems": 5
          },
          "symmetryOpsD": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSymmetryOpD5"
            },
            "minItems": 1
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSiteD5"
            },
            "minItems": 0
          },
          "occupationDomains": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainDefinition"
            },
            "minItems": 0
          },
          "nativeSourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          },
          "occupationDomainTemplates": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainTemplate"
            },
            "minItems": 0
          }
        },
        "required": [
          "modelId",
          "dimensionD",
          "nativeMatrixDimensionWord",
          "nativeModelKind",
          "sourceGroupNumberI32",
          "sourceSettingOrdinalI32",
          "crystallographicToOrthoRowsD",
          "orthoToCrystallographicRowsD",
          "phasonRowsD",
          "layerWorldMatrix4Rows",
          "currentC2XYZRowsDp1",
          "currentXYZ2CRowsDp1",
          "nativeAuxInverse1A0RowsDp1",
          "qcorigNative6",
          "nativeCIFWaveVectorRows3",
          "drawingBoundsMinMaxD",
          "symmetryOpsD",
          "sites",
          "occupationDomains",
          "nativeSourceOrder",
          "source",
          "occupationDomainTemplates"
        ],
        "x-producer-consumer": "03native-source/document actualcurrentD capture -> Scene::BuildOrientMatrix -> BuildCrystal2(D3)/BuildCluster(D4)/BuildQuasiCrystal(D5/6) -> physical display/scatter/currenttypedarchive",
        "x-invariants": [
          "dimensionD isactualmathematicalD;Crystal+168 storesD+1,notD;nativeQUASICRYSTAL DIM=constructor receivesD",
          "C2O/O2C fromGetMatrixC2O/+510 andGetMatrixO2C/+538;fullDmatrix retained;PHASON at+588;QCORIG at+78..a0 sixsourcef64",
          "GetMatrixC2XYZ+150/GetMatrixXYZ2C+178 returnfull(D+1)^2;notonly16values forD>3",
          "BuildOrientMatrix: temp=PHASON*C2O;Initialize copiesmin dimensions intoexistingC2XYZ withoutresize;GeMM(M4,temp)updatesM=min(D+1,4),N=D,K=min(4,D),otherrows/edgevaluesretainactualpreviousstate;translationcolD first3 updatedbyindependentpositionreference rule",
          "InitializeI buildsXYZ2C inverse;nativeAux1A0 copiesinverse andnegateslastcolumnfirst3;fullsourceedgewords retained,not implicitzero padding",
          "higherD positions usecurrentDops andcompleteDcoordinates;projectphysicalfirst3onlyafterD-affine evaluation;internalD-3 componentsremainmodel/identity",
          "nativeCIFwaveq rows storedordered3;do notassignunprovedharmonicfunctionmeaning;actualhigherDstructurefactors useDdotproduct perFS evidence",
          "allsite/domain/opids resolvewithinsameowner/revision;actualDcoordinates survivetypedarchive/change/restore; no source-onlydowngrade",
          "nativefileprojection/save usesactualqualifiedgrammar/full loss report;internalenginearchivepersists alltypedDfields andsource independently;never stripsinternalaxes",
          "scene builddispatchnativeword4→D3BuildCrystal2,word5→D4BuildCluster,elseD5/6BuildQuasiCrystal;runtimequalificationnot virtual exclusion",
          "templateOrdinal resolves Crystal+1f8 sourcegeometry independentlyof per-sitecopy;siteOD currentvertices mayalreadytransformed;never usemutatedcurrentcopy assource template"
        ]
      },
      "VectorD6": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "MatrixD6Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/VectorD6"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "MatrixD7HomogeneousRows": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "minItems": 7,
          "maxItems": 7
        },
        "minItems": 7,
        "maxItems": 7
      },
      "NativeSiteD6": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "nativeSpeciesToken": {
            "type": "string",
            "pattern": "^[!-~]{1,2}$"
          },
          "nativeElementIndexI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          },
          "fractionalCoordinatesD": {
            "$ref": "#/$defs/VectorD6"
          },
          "occupancySlotD": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "nativeScalarSlotD1": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "sourceUorBangstrom^2"
          },
          "thermal6SlotsD2ToD7": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "coordinateSUD": {
            "$ref": "#/$defs/VectorD6"
          },
          "nativeSiteArrayBlocks3ByDPlus8": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 14,
              "maxItems": 14
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeFlagsU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "occupationDomainId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "nativeOrbitNumeratorI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          }
        },
        "required": [
          "siteId",
          "nativeSpeciesToken",
          "nativeElementIndexI16",
          "fractionalCoordinatesD",
          "occupancySlotD",
          "nativeScalarSlotD1",
          "thermal6SlotsD2ToD7",
          "coordinateSUD",
          "nativeSiteArrayBlocks3ByDPlus8",
          "nativeFlagsU16",
          "occupationDomainId",
          "nativeOrbitNumeratorI16"
        ],
        "x-invariants": [
          "XSite+18=6;allocated3*(6+8)f64 @c6f50..c6f80",
          "explicitcoordinates/scalar/tensor/SU mustequalactualblockprojections;block0[:D]=coordinates,block0[D]=occupancy,block0[D+1]=scalar,block0[D+2:D+8]=thermal6,block1[:D]=coordinateSU",
          "nativeFlagsU16 isactualXSite+da,bit3 usedbyactualscatter path;not replaced byglobal THERT/THERI/table flags",
          "block2 andunconsumedblockfields exacttypedcurrentdata,not imaginaryharmoniccoefficients; originalsourceobservationindependent",
          "nativeOrbitNumeratorI16 is signedXSite+d8 used byactualSF/densityweight;do notdecodeunsigned merelybecause storage16bits"
        ]
      },
      "NativeSymmetryOpD6": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "currentNativeMatrixRows": {
            "$ref": "#/$defs/MatrixD7HomogeneousRows"
          },
          "timeSign": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "operatorId",
          "currentNativeMatrixRows",
          "timeSign"
        ],
        "x-invariants": [
          "positionaffine usesfirstDrows/cols pluscolDtranslation;nativebottomrowandtime representation keptactual,not rewrittenasordinaryhomogeneous1",
          "currentUnitCell+70 fullDoplist,not3Dtruncate ofgenerator/sourcehistory"
        ]
      },
      "NativeDimensionD6State": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "modelId": {
            "$ref": "#/$defs/Id"
          },
          "dimensionD": {
            "const": 6
          },
          "nativeMatrixDimensionWord": {
            "const": 7
          },
          "nativeModelKind": {
            "enum": [
              "crystal",
              "molecule",
              "quasicrystal",
              "magnetic-quasicrystal"
            ]
          },
          "sourceGroupNumberI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "sourceSettingOrdinalI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "crystallographicToOrthoRowsD": {
            "$ref": "#/$defs/MatrixD6Rows"
          },
          "orthoToCrystallographicRowsD": {
            "$ref": "#/$defs/MatrixD6Rows"
          },
          "phasonRowsD": {
            "$ref": "#/$defs/MatrixD6Rows"
          },
          "layerWorldMatrix4Rows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "currentC2XYZRowsDp1": {
            "$ref": "#/$defs/MatrixD7HomogeneousRows"
          },
          "currentXYZ2CRowsDp1": {
            "$ref": "#/$defs/MatrixD7HomogeneousRows"
          },
          "nativeAuxInverse1A0RowsDp1": {
            "$ref": "#/$defs/MatrixD7HomogeneousRows"
          },
          "qcorigNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "nativeCIFWaveVectorRows3": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "drawingBoundsMinMaxD": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -3.4028234663852886e+38,
                "maximum": 3.4028234663852886e+38,
                "x-unit": "native-fractional-bound",
                "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
              },
              "minItems": 2,
              "maxItems": 2
            },
            "minItems": 6,
            "maxItems": 6
          },
          "symmetryOpsD": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSymmetryOpD6"
            },
            "minItems": 1
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSiteD6"
            },
            "minItems": 0
          },
          "occupationDomains": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainDefinition"
            },
            "minItems": 0
          },
          "nativeSourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          },
          "occupationDomainTemplates": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeOccupationDomainTemplate"
            },
            "minItems": 0
          }
        },
        "required": [
          "modelId",
          "dimensionD",
          "nativeMatrixDimensionWord",
          "nativeModelKind",
          "sourceGroupNumberI32",
          "sourceSettingOrdinalI32",
          "crystallographicToOrthoRowsD",
          "orthoToCrystallographicRowsD",
          "phasonRowsD",
          "layerWorldMatrix4Rows",
          "currentC2XYZRowsDp1",
          "currentXYZ2CRowsDp1",
          "nativeAuxInverse1A0RowsDp1",
          "qcorigNative6",
          "nativeCIFWaveVectorRows3",
          "drawingBoundsMinMaxD",
          "symmetryOpsD",
          "sites",
          "occupationDomains",
          "nativeSourceOrder",
          "source",
          "occupationDomainTemplates"
        ],
        "x-producer-consumer": "03native-source/document actualcurrentD capture -> Scene::BuildOrientMatrix -> BuildCrystal2(D3)/BuildCluster(D4)/BuildQuasiCrystal(D5/6) -> physical display/scatter/currenttypedarchive",
        "x-invariants": [
          "dimensionD isactualmathematicalD;Crystal+168 storesD+1,notD;nativeQUASICRYSTAL DIM=constructor receivesD",
          "C2O/O2C fromGetMatrixC2O/+510 andGetMatrixO2C/+538;fullDmatrix retained;PHASON at+588;QCORIG at+78..a0 sixsourcef64",
          "GetMatrixC2XYZ+150/GetMatrixXYZ2C+178 returnfull(D+1)^2;notonly16values forD>3",
          "BuildOrientMatrix: temp=PHASON*C2O;Initialize copiesmin dimensions intoexistingC2XYZ withoutresize;GeMM(M4,temp)updatesM=min(D+1,4),N=D,K=min(4,D),otherrows/edgevaluesretainactualpreviousstate;translationcolD first3 updatedbyindependentpositionreference rule",
          "InitializeI buildsXYZ2C inverse;nativeAux1A0 copiesinverse andnegateslastcolumnfirst3;fullsourceedgewords retained,not implicitzero padding",
          "higherD positions usecurrentDops andcompleteDcoordinates;projectphysicalfirst3onlyafterD-affine evaluation;internalD-3 componentsremainmodel/identity",
          "nativeCIFwaveq rows storedordered3;do notassignunprovedharmonicfunctionmeaning;actualhigherDstructurefactors useDdotproduct perFS evidence",
          "allsite/domain/opids resolvewithinsameowner/revision;actualDcoordinates survivetypedarchive/change/restore; no source-onlydowngrade",
          "nativefileprojection/save usesactualqualifiedgrammar/full loss report;internalenginearchivepersists alltypedDfields andsource independently;never stripsinternalaxes",
          "scene builddispatchnativeword4→D3BuildCrystal2,word5→D4BuildCluster,elseD5/6BuildQuasiCrystal;runtimequalificationnot virtual exclusion",
          "templateOrdinal resolves Crystal+1f8 sourcegeometry independentlyof per-sitecopy;siteOD currentvertices mayalreadytransformed;never usemutatedcurrentcopy assource template"
        ]
      },
      "NativeDimensionDState": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeDimensionD3State"
          },
          {
            "$ref": "#/$defs/NativeDimensionD4State"
          },
          {
            "$ref": "#/$defs/NativeDimensionD5State"
          },
          {
            "$ref": "#/$defs/NativeDimensionD6State"
          }
        ]
      },
      "ProjectedDImage": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "translationIntegerD": {
            "type": "array",
            "items": {
              "type": "integer"
            },
            "minItems": 3
          },
          "fractionalImageD": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 3
          },
          "nativeXYZImageD": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "native-coordinate"
            },
            "minItems": 3
          },
          "physicalCartesianA": {
            "$ref": "#/$defs/Vec3A"
          },
          "insideDrawingBoundary": {
            "type": "boolean"
          },
          "occupationDomainId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "siteId",
          "operatorId",
          "translationIntegerD",
          "fractionalImageD",
          "nativeXYZImageD",
          "physicalCartesianA",
          "insideDrawingBoundary",
          "occupationDomainId"
        ],
        "x-invariants": [
          "allDarrays length=state.dimensionD;no generic3Dcoercion",
          "R_D*f_D+t_D followedbycurrentC2XYZfullDaffine;physicalCartesianA equalsfirst3 nativeCartesiancomponents afteractualfullDtransform",
          "tuple(siteId,operatorId,translationIntegerD) canonicalimage identity;specialpositionaliases representedexplicitly notmergingbylabel"
        ]
      },
      "DomainDisplayMesh": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "domainId": {
            "$ref": "#/$defs/Id"
          },
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "operatorId": {
            "$ref": "#/$defs/Id"
          },
          "verticesCartesianA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3A"
            },
            "minItems": 0
          },
          "facesVertexIndices": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              },
              "minItems": 3
            },
            "minItems": 0
          },
          "sourceVertexOrbits": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            },
            "minItems": 0
          }
        },
        "required": [
          "domainId",
          "siteId",
          "operatorId",
          "verticesCartesianA",
          "facesVertexIndices",
          "sourceVertexOrbits"
        ],
        "x-invariants": [
          "allfaceindices inrange;drawnODphysicalmesh isderivedfromfullnativeDdefinition/currenttransform,notpersisted source3Dreplacement",
          "nativeDrawODsImpl algorithmprofile/sourceSHA determinesdomainplanes/clip/order;explicitmeshdoesnotproveunrunnativepixels"
        ]
      },
      "NativeDimensionDRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/NativeDimensionDState"
          },
          "action": {
            "enum": [
              "project-current-sites",
              "build-native-display",
              "capture-for-restore"
            ]
          },
          "maximumImages": {
            "type": "integer",
            "minimum": 1
          },
          "maximumDomainVertices": {
            "type": "integer",
            "minimum": 1
          }
        },
        "required": [
          "state",
          "action",
          "maximumImages",
          "maximumDomainVertices"
        ]
      },
      "NativeDimensionDResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "currentState": {
            "$ref": "#/$defs/NativeDimensionDState"
          },
          "images": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ProjectedDImage"
            },
            "minItems": 0
          },
          "domainMeshes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/DomainDisplayMesh"
            },
            "minItems": 0
          },
          "completion": {
            "enum": [
              "complete",
              "bounded-partial"
            ]
          },
          "remainingFrontierCount": {
            "type": "integer",
            "minimum": 0
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "enum": [
                "source-reference-missing",
                "singular-native-transform",
                "nonfinite-scientific-input",
                "image-budget-reached",
                "mesh-budget-reached",
                "native-invalid-domain"
              ]
            },
            "minItems": 0
          }
        },
        "required": [
          "currentState",
          "images",
          "domainMeshes",
          "completion",
          "remainingFrontierCount",
          "diagnostics"
        ],
        "x-invariants": [
          "completioncomplete iff noadmittedunexpandedimages/domainvertices;boundsbudgetpartialneverclaimscomplete",
          "capture-for-restore restoresactualDtypedstate,allsite/ODrelations,flags,currentqcorig/matrices/sourceidentity before projection"
        ]
      },
      "Diagnostic": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "code": {
            "enum": [
              "E_SOURCE_REFERENCE",
              "E_DIMENSION",
              "E_SINGULAR_TRANSFORM",
              "E_NONFINITE",
              "E_UNDERSPECIFIED",
              "E_INVALID_COVARIANCE",
              "E_NATIVE_SOURCE_MISMATCH",
              "E_RESOURCE_LIMIT",
              "E_DEGENERATE_GEOMETRY",
              "E_AMBIGUOUS_BV_ROW",
              "E_RESOURCE_UNAVAILABLE"
            ]
          },
          "entityId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "detail": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "code",
          "entityId",
          "detail"
        ]
      },
      "ResolveNativePlacementsRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/PhasePlacementSet"
          }
        },
        "required": [
          "state"
        ]
      },
      "ResolveNativePlacementsResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "resolved": {
            "$ref": "#/$defs/PhasePlacementSet"
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "resolved",
          "diagnostics"
        ]
      },
      "NativeGroupRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/GroupSettingState"
          },
          "sitesFractional3": {
            "type": "array",
            "items": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "siteId": {
                  "$ref": "#/$defs/Id"
                },
                "position": {
                  "$ref": "#/$defs/Vec3"
                }
              },
              "required": [
                "siteId",
                "position"
              ]
            },
            "minItems": 0
          }
        },
        "required": [
          "state",
          "sitesFractional3"
        ]
      },
      "NativeGroupResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/GroupSettingState"
          },
          "expanded": {
            "type": "array",
            "items": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "siteId": {
                  "$ref": "#/$defs/Id"
                },
                "operatorId": {
                  "$ref": "#/$defs/Id"
                },
                "fractionalImageModulo1": {
                  "$ref": "#/$defs/Vec3"
                }
              },
              "required": [
                "siteId",
                "operatorId",
                "fractionalImageModulo1"
              ]
            },
            "minItems": 0
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "state",
          "expanded",
          "diagnostics"
        ]
      },
      "NativeWyckoffRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/GroupSettingState"
          },
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "positionFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "coordinateSUFractional": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "state",
          "siteId",
          "positionFractional",
          "coordinateSUFractional"
        ]
      },
      "NativeWyckoffResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "siteId": {
            "$ref": "#/$defs/Id"
          },
          "assignment": {
            "oneOf": [
              {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                  "kind": {
                    "const": "assigned"
                  },
                  "multiplicity": {
                    "type": "integer",
                    "minimum": 1
                  },
                  "letter": {
                    "type": "string",
                    "pattern": "^[A-Za-z]$"
                  },
                  "sourceRow": {
                    "$ref": "#/$defs/SourceSpan"
                  },
                  "effectiveToleranceFractional": {
                    "$ref": "#/$defs/Vec3"
                  }
                },
                "required": [
                  "kind",
                  "multiplicity",
                  "letter",
                  "sourceRow",
                  "effectiveToleranceFractional"
                ]
              },
              {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                  "kind": {
                    "const": "unassigned"
                  },
                  "reason": {
                    "enum": [
                      "missing-source-row",
                      "no-compatible-position",
                      "invalid-source-setting"
                    ]
                  }
                },
                "required": [
                  "kind",
                  "reason"
                ]
              }
            ]
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "siteId",
          "assignment",
          "diagnostics"
        ]
      },
      "CovarianceParameter": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "parameterId": {
            "type": "string",
            "minLength": 1
          },
          "unit": {
            "enum": [
              "fractional",
              "angstrom",
              "degree",
              "angstrom^2",
              "dimensionless"
            ]
          },
          "centralValue": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          }
        },
        "required": [
          "parameterId",
          "unit",
          "centralValue"
        ]
      },
      "UncertaintyModel": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "authority": {
            "enum": [
              "native-diagonal-approximation",
              "supplied-full-covariance"
            ]
          },
          "parametersInOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/CovarianceParameter"
            },
            "minItems": 1
          },
          "covarianceRows": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 1
            },
            "minItems": 1
          },
          "independentParameterIds": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            },
            "minItems": 0
          },
          "fullParameterByIndependentJacobianRows": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 0
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "authority",
          "parametersInOrder",
          "covarianceRows",
          "independentParameterIds",
          "fullParameterByIndependentJacobianRows",
          "source"
        ],
        "x-invariants": [
          "covariancedimensionN matchesparameterorder;matrixsymmetricPSD;eachentryunitsproductofcorrespondingparameterunits",
          "nativediagonalapproximationexplicitlyignorescrosscorrelations;absent isnot measuredzero",
          "constraintfull=T*independent;J_ind=J_full*T,C_full=T*C_ind*T^T;sourceconstraintrevision/parameterorderingchecked"
        ]
      },
      "NativeMeasurement": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "kind": {
            "enum": [
              "distance",
              "angle",
              "torsion",
              "plane-angle"
            ]
          },
          "selectedCartesianPointsA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3A"
            },
            "minItems": 0
          },
          "selectedImageIds": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 0
          },
          "parameterToSelectedCartesianJacobianRows": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              "minItems": 0
            },
            "minItems": 0
          }
        },
        "required": [
          "kind",
          "selectedCartesianPointsA",
          "selectedImageIds",
          "parameterToSelectedCartesianJacobianRows"
        ],
        "x-invariants": [
          "distance2points,angle3,torsion4;plane-angle explicittwo3pointplanes;imageids matchpoints",
          "Jacobian rows3*pointCount byN parameters,alreadyincludesactualH/symmetry/phase/constraint transforms;cellangledegree derivatives includeπ/180",
          "no uncertaintyzero default atzero-distance/collinearity;typedundefined withcentral geometrydiagnostic"
        ]
      },
      "NativeADPAndUncertaintyRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "adpPlan": {
            "$ref": "#/$defs/AdpRenderPlan"
          },
          "measurement": {
            "$ref": "#/$defs/NativeMeasurement"
          },
          "uncertainty": {
            "$ref": "#/$defs/UncertaintyModel"
          }
        },
        "required": [
          "adpPlan",
          "measurement",
          "uncertainty"
        ]
      },
      "NativeADPAndUncertaintyResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "adpPlan": {
            "$ref": "#/$defs/AdpRenderPlan"
          },
          "measurementValue": {
            "oneOf": [
              {
                "type": "number",
                "minimum": -1.7976931348623157e+308,
                "maximum": 1.7976931348623157e+308,
                "x-unit": "dimensionless"
              },
              {
                "type": "null"
              }
            ]
          },
          "measurementUnit": {
            "enum": [
              "angstrom",
              "degree"
            ]
          },
          "uncertaintyResult": {
            "oneOf": [
              {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                  "kind": {
                    "const": "defined"
                  },
                  "jacobianInParameterOrder": {
                    "type": "array",
                    "items": {
                      "type": "number",
                      "minimum": -1.7976931348623157e+308,
                      "maximum": 1.7976931348623157e+308,
                      "x-unit": "dimensionless"
                    },
                    "minItems": 1
                  },
                  "variance": {
                    "type": "number",
                    "minimum": 0,
                    "maximum": 1.7976931348623157e+308
                  },
                  "standardUncertainty": {
                    "type": "number",
                    "minimum": 0,
                    "maximum": 1.7976931348623157e+308
                  }
                },
                "required": [
                  "kind",
                  "jacobianInParameterOrder",
                  "variance",
                  "standardUncertainty"
                ]
              },
              {
                "type": "object",
                "additionalProperties": false,
                "properties": {
                  "kind": {
                    "const": "undefined"
                  },
                  "reason": {
                    "enum": [
                      "zero-distance-gradient",
                      "collinear-angle",
                      "underdetermined-plane",
                      "invalid-covariance"
                    ]
                  }
                },
                "required": [
                  "kind",
                  "reason"
                ]
              }
            ]
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "adpPlan",
          "measurementValue",
          "measurementUnit",
          "uncertaintyResult",
          "diagnostics"
        ],
        "x-invariants": [
          "measurementValue null exactlywhencentralgeometryitselfisundefined(e.gcollineartorsion);value0forzero-distance remainsvalidcentralvaluebutitsSUgradientundefined",
          "centralvalueandSUdefinedness areindependent;neverfabricatefinite0 forundefinedcentralangle/torsion;typedundefinedSUreason+Diagnostic identifiesgeometry"
        ]
      },
      "NativeMergeRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "frame": {
            "$ref": "#/$defs/Frame"
          },
          "profile": {
            "enum": [
              "native-greedy-source-order",
              "detect-connected-components",
              "corrected-metric-greedy"
            ]
          },
          "thresholdA": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 3.4028234663852886e+38
          },
          "sourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            },
            "minItems": 1
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MergeSiteInput"
            },
            "minItems": 1
          },
          "symmetryOps": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SymmetryOp3"
            },
            "minItems": 1
          },
          "bondRules": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PublicBondRule"
            },
            "minItems": 0
          }
        },
        "required": [
          "phaseId",
          "frame",
          "profile",
          "thresholdA",
          "sourceOrder",
          "sites",
          "symmetryOps",
          "bondRules"
        ]
      },
      "NativeMergeResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "groups": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MergeGroupResult"
            },
            "minItems": 0
          },
          "mutatesSites": {
            "type": "boolean"
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "groups",
          "mutatesSites",
          "diagnostics"
        ],
        "x-invariants": [
          "detect-only profilemutatesSites=false;nativegreedy/sourceSUmetadata rules fromMergePlan;engineapply/preview owns actualtransaction"
        ]
      },
      "BondValenceRow": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "rowKey": {
            "type": "string",
            "minLength": 1
          },
          "assetSHA256": {
            "$ref": "#/$defs/SHA256"
          },
          "speciesA": {
            "type": "string",
            "minLength": 1
          },
          "speciesB": {
            "type": "string",
            "minLength": 1
          },
          "oxidationA": {
            "type": "integer"
          },
          "oxidationB": {
            "type": "integer"
          },
          "r0A": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "angstrom"
          },
          "bA": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "rowKey",
          "assetSHA256",
          "speciesA",
          "speciesB",
          "oxidationA",
          "oxidationB",
          "r0A",
          "bA",
          "source"
        ]
      },
      "NativePlaneRequest": {
        "$ref": "#/$defs/PlaneFitInput"
      },
      "NativePlaneResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "result": {
            "oneOf": [
              {
                "$ref": "#/$defs/PlaneFitResolved"
              },
              {
                "$ref": "#/$defs/PlaneFitUnderdetermined"
              }
            ]
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "result",
          "diagnostics"
        ]
      },
      "Halfspace3": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "halfspaceId": {
            "$ref": "#/$defs/Id"
          },
          "normalInverseA": {
            "$ref": "#/$defs/Vec3"
          },
          "offsetDimensionless": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          }
        },
        "required": [
          "halfspaceId",
          "normalInverseA",
          "offsetDimensionless"
        ],
        "x-invariants": [
          "n dot rCartesianA<=offset;zero-normal separatelyclassifiable,orinputdiagnostic perprofile",
          "allfinitef64 convertedexactbinaryrational forcorrectedclassification;noepsilon topologychanges"
        ]
      },
      "ExplicitMesh3": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "verticesCartesianA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3A"
            },
            "minItems": 0
          },
          "facesVertexIndices": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              },
              "minItems": 3
            },
            "minItems": 0
          }
        },
        "required": [
          "verticesCartesianA",
          "facesVertexIndices"
        ],
        "x-invariants": [
          "indicesinrange,consistentwinding,closedorientedmanifold andselfintersectionvalidated beforevolume/inside",
          "nonconvexexplicitfaces meaningful;unorderedpointsalone donotdefineunique nonconvexsolid"
        ]
      },
      "NativeShapeRequest": {
        "oneOf": [
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "profile": {
                "const": "corrected-exact-halfspaces"
              },
              "halfspaces": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Halfspace3"
                },
                "minItems": 0
              }
            },
            "required": [
              "profile",
              "halfspaces"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "profile": {
                "const": "explicit-nonconvex-mesh"
              },
              "mesh": {
                "$ref": "#/$defs/ExplicitMesh3"
              }
            },
            "required": [
              "profile",
              "mesh"
            ]
          },
          {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "profile": {
                "const": "native-convex-neighbor-hull"
              },
              "neighborImageIds": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Id"
                },
                "minItems": 0
              },
              "neighborCartesianA": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Vec3A"
                },
                "minItems": 0
              }
            },
            "required": [
              "profile",
              "neighborImageIds",
              "neighborCartesianA"
            ]
          }
        ]
      },
      "NativeShapeResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "classification": {
            "enum": [
              "empty",
              "point",
              "segment",
              "polygon",
              "solid",
              "unbounded",
              "invalid-mesh"
            ]
          },
          "affineDimension": {
            "type": "integer",
            "minimum": 0,
            "maximum": 3
          },
          "verticesCartesianA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3A"
            },
            "minItems": 0
          },
          "exactVerticesA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Rational3"
            },
            "minItems": 0
          },
          "facesVertexIndices": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0
              },
              "minItems": 3
            },
            "minItems": 0
          },
          "farkasMultipliers": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Rational"
            },
            "minItems": 0
          },
          "feasiblePointA": {
            "oneOf": [
              {
                "$ref": "#/$defs/Rational3"
              },
              {
                "type": "null"
              }
            ]
          },
          "recessionDirections": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Rational3"
            },
            "minItems": 0
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "classification",
          "affineDimension",
          "verticesCartesianA",
          "exactVerticesA",
          "facesVertexIndices",
          "farkasMultipliers",
          "feasiblePointA",
          "recessionDirections",
          "diagnostics"
        ],
        "x-invariants": [
          "empty requiresnonnegativey withA^T*y=0,b^T*y<0;unboundedrequiresnonzerorecessionrayA*r<=0",
          "affinerank exact;boundedvertices allactiveintersectionscheckedallinequalities;nozero-volume==empty shortcut",
          "nativehull preservesactualarea0.002/coplanar0.009999999776482582 andSVD/order profile;machineparity futurequalification",
          "allcertificatearraysdiagnosticallyspecific;emptyarraysonlywherecertificateclassdoesnotneedthem"
        ]
      },
      "NativeCameraRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/NativeCamera"
          },
          "pointsCartesianA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3A"
            },
            "minItems": 0
          }
        },
        "required": [
          "state",
          "pointsCartesianA"
        ]
      },
      "NativeCameraResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/NativeCamera"
          },
          "modelviewRows": {
            "$ref": "#/$defs/Matrix4Rows"
          },
          "projectedNDC": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 0
          },
          "physicalViewportPixels": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1
            },
            "minItems": 2,
            "maxItems": 2
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "state",
          "modelviewRows",
          "projectedNDC",
          "physicalViewportPixels",
          "diagnostics"
        ],
        "x-invariants": [
          "perspective homogeneousdivide onlynonzerow,clip statusdiagnosed;sourceZscale not fov",
          "samegeometryrevision/viewport/deviceScale;GL/pixelqualification notasserted bymatrixresult"
        ]
      },
      "ApplyNativeDomainParametersRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/NativeDimensionDState"
          },
          "domainId": {
            "$ref": "#/$defs/Id"
          },
          "xeA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "angstrom"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "parameterFlagsI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          }
        },
        "required": [
          "state",
          "domainId",
          "xeA",
          "parameterFlagsI32"
        ]
      },
      "ApplyNativeDomainParametersResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "state": {
            "$ref": "#/$defs/NativeDimensionDState"
          },
          "domainId": {
            "$ref": "#/$defs/Id"
          },
          "derivedDomainTranslationNative6": {
            "$ref": "#/$defs/Vec6"
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "state",
          "domainId",
          "derivedDomainTranslationNative6",
          "diagnostics"
        ]
      },
      "NativeOccupationDomainTemplate": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "templateOrdinal0based": {
            "type": "integer",
            "minimum": 0
          },
          "coordinateWidth": {
            "const": 6
          },
          "verticesNative6": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec6"
            },
            "minItems": 0
          },
          "nativeNthVertexIndices0based": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "templateOrdinal0based",
          "coordinateWidth",
          "verticesNative6",
          "nativeNthVertexIndices0based",
          "source"
        ],
        "x-producer-consumer": "readdata shape registryCrystal+1f8 -> BuildQuasiCrystal191d0/19248:copytemplatevertices ->currentODgeometry;sourceimmutable andtypedcurrentindependent",
        "x-invariants": [
          "nthpairsgroup2 forD5 ortriplesgroup3 forD6 indexvertices,not symmetryoperations",
          "sourceoriginal6vectors remainsource;actualD5read converts6to5 beforecurrentmodelscientificevaluation"
        ]
      },
      "NativeTableBVRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "profile": {
            "enum": [
              "explicit-bond-valence-sum",
              "native-auto-cutoff"
            ]
          },
          "candidateRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/BondValenceRow"
            },
            "minItems": 1
          },
          "selectedRowKeys": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            },
            "minItems": 0
          },
          "distanceA": {
            "type": "number",
            "minimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "nativeCutoffDenominator": {
            "type": "integer",
            "minimum": 1
          },
          "denominatorSource": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "profile",
          "candidateRows",
          "selectedRowKeys",
          "distanceA",
          "nativeCutoffDenominator",
          "denominatorSource"
        ],
        "x-invariants": [
          "explicitBVS rowchoice mustbefullyresolved;oxidation/species/resource identitycannotbeguessed",
          "native-auto-cutoff: rowcut=R0−b*log(abs(oxidationA)/nativeDenominator);pairmaximum then+nativef640.2 andf32 rounding;not selectedBVS row",
          "no implicitfirstrow/mixededition;source denominator table selector mustbequalifiedexactasactualnativegroupordinal",
          "SelectedReadBVParm literal namesbvparm2013.cif. PackagedA05bvparm2020.cif is distinctasset;absence of2013doesnotprovewholeappdefaultfailure orreachability. Functionmissingresource/cachedparameters trackedexplicitly;never silentresourceeditionfallback."
        ]
      },
      "NativePolyhedronBVSRequest": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "profile": {
            "const": "native-polyhedron-output"
          },
          "centerSiteId": {
            "$ref": "#/$defs/Id"
          },
          "centerOccupancyF64": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "userR0F32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "angstrom",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "sourceOutputFlagsU32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "neighborsInSourceOrder": {
            "type": "array",
            "items": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "imageId": {
                  "$ref": "#/$defs/Id"
                },
                "neighborSiteId": {
                  "$ref": "#/$defs/Id"
                },
                "distanceA": {
                  "type": "number",
                  "minimum": 0,
                  "maximum": 1.7976931348623157e+308
                }
              },
              "required": [
                "imageId",
                "neighborSiteId",
                "distanceA"
              ]
            },
            "minItems": 0
          },
          "source": {
            "$ref": "#/$defs/SourceSpan"
          }
        },
        "required": [
          "profile",
          "centerSiteId",
          "centerOccupancyF64",
          "userR0F32",
          "sourceOutputFlagsU32",
          "neighborsInSourceOrder",
          "source"
        ],
        "x-invariants": [
          "sourceOutputFlagsU32 bit1 requests thisnativeBVSreport;engine typedoperationdoesnotpromptcallbacks",
          "actualnativeprompt parsesR0 with%f ->binary32;explicitcurrentvalue retained,promotedf64forarithmetic",
          "nativeb exactf64 bits3fd7ae147ae147ae(0.37);neighborlist fixedactualpolyhedron+78 sourcelist andcorrespondingdistancearray,notallbondslabelmatch",
          "term_j=exp((promoteF32(R0)-distanceF64_j)/bF64);printedNeighborTerm_j=-term_j;accum_j=fma(term_j,centerOccupancyF64,accum_j-1),initial0;useCENTERoccupancy not neighbor occupancy",
          "noassetrow lookup inthisactualbranch;nativeReadBVParm2013 auto-cutoff helper andexplicit2020BVS areseparateprofiles"
        ]
      },
      "NativeBVRequest": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeTableBVRequest"
          },
          {
            "$ref": "#/$defs/NativePolyhedronBVSRequest"
          }
        ]
      },
      "NativeTableBVResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "selectedRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/BondValenceRow"
            },
            "minItems": 0
          },
          "terms": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -1.7976931348623157e+308,
              "maximum": 1.7976931348623157e+308,
              "x-unit": "dimensionless"
            },
            "minItems": 0
          },
          "sum": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "nativeAutoCutoffA": {
            "oneOf": [
              {
                "type": "number",
                "minimum": -3.4028234663852886e+38,
                "maximum": 3.4028234663852886e+38,
                "x-unit": "angstrom",
                "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
              },
              {
                "type": "null"
              }
            ]
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "selectedRows",
          "terms",
          "sum",
          "nativeAutoCutoffA",
          "diagnostics"
        ],
        "x-invariants": [
          "BVSterm=exp((R0−distance)/b);nativecutofffieldnullforBVSprofile;realambiguousselection returnsdiagnostic,notguess"
        ]
      },
      "NativePolyhedronBVSResponse": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "profile": {
            "const": "native-polyhedron-output"
          },
          "centerSiteId": {
            "$ref": "#/$defs/Id"
          },
          "bF64": {
            "const": 0.37
          },
          "neighborsInSourceOrder": {
            "type": "array",
            "items": {
              "type": "object",
              "additionalProperties": false,
              "properties": {
                "imageId": {
                  "$ref": "#/$defs/Id"
                },
                "neighborSiteId": {
                  "$ref": "#/$defs/Id"
                },
                "distanceA": {
                  "type": "number",
                  "minimum": -1.7976931348623157e+308,
                  "maximum": 1.7976931348623157e+308,
                  "x-unit": "angstrom"
                },
                "positiveTerm": {
                  "type": "number",
                  "minimum": -1.7976931348623157e+308,
                  "maximum": 1.7976931348623157e+308,
                  "x-unit": "dimensionless"
                },
                "printedNegativeTerm": {
                  "type": "number",
                  "minimum": -1.7976931348623157e+308,
                  "maximum": 1.7976931348623157e+308,
                  "x-unit": "dimensionless"
                }
              },
              "required": [
                "imageId",
                "neighborSiteId",
                "distanceA",
                "positiveTerm",
                "printedNegativeTerm"
              ]
            },
            "minItems": 0
          },
          "centerOccupancyWeightedSum": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "dimensionless"
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            },
            "minItems": 0
          }
        },
        "required": [
          "profile",
          "centerSiteId",
          "bF64",
          "neighborsInSourceOrder",
          "centerOccupancyWeightedSum",
          "diagnostics"
        ],
        "x-invariants": [
          "reportedperneighborprintedvalue hasnegative sign;finalprintedBVSsum ispositivecenteroccupancyweightedaccumulation",
          "sourceoperationorder/fmaqualification separatefrommathematicalfunctionality;displayformat%gterms/%6.3fsum doesnotreplacefullf64currentresult"
        ]
      },
      "NativeBVResponse": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeTableBVResponse"
          },
          {
            "$ref": "#/$defs/NativePolyhedronBVSResponse"
          }
        ]
      }
    },
    "x-wire-registration": false,
    "x-existing-acceptance-mutated": false,
    "x-producer-consumer-contract": "sg2-producer-consumer.json",
    "x-operation-signatures": "sg2-operations.json"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "draft.fs2-scientific-successor.schema.json",
    "title": "FS/2 closed source-consumer and explicit mathematical design payloads",
    "$ref": "#/$defs/BoundaryRequest",
    "$defs": {
      "FS1_Id": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Id"
      },
      "FS1_Digest": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Digest"
      },
      "FS1_FSContext": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSContext"
      },
      "FS1_FSByteSpan": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSByteSpan"
      },
      "FS1_FSFieldBinding": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldBinding"
      },
      "FS1_FSFieldOutput": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldOutput"
      },
      "FS1_FSSpectrumInput": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSSpectrumInput"
      },
      "FS1_FSSpectrumResult": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSSpectrumResult"
      },
      "FS1_FSProjectionInput": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSProjectionInput"
      },
      "FS1_FSProjectionResult": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSProjectionResult"
      },
      "FS1_FSSliceResult": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSSliceResult"
      },
      "FS1_FSProvenance": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSProvenance"
      },
      "FS1_FSTableUse": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSTableUse"
      },
      "FS1_Species": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Species"
      },
      "FS1_SGTableIdentity": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/SGTableIdentity"
      },
      "FS1_Cell": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Cell"
      },
      "FS1_Int3": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Int3"
      },
      "FS1_Vec3": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Vec3"
      },
      "FS1_Mat3": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Mat3"
      },
      "FS1_Curve": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Curve"
      },
      "FS1_Diagnostic": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Diagnostic"
      },
      "FC_NativeFractionalGridCurrent1": {
        "$ref": "../../../../../20261115-092025-p1-03-completion/native-fractional-grid-current.schema.json"
      },
      "FC_NativeScalarGridCurrent1": {
        "$ref": "../../../../../20261115-092025-p1-03-completion/native-scalar-grid-current.schema.json"
      },
      "FC_NativeStoredScalarIsoview1": {
        "$ref": "../../../../../20261115-092025-p1-03-completion/native-stored-isoview.schema.json"
      },
      "Math_FSResampleAffineInput": {
        "$ref": "functional-additions.schema.json#/$defs/FSResampleAffineInput"
      },
      "Math_FSLinearCombineFieldsInput": {
        "$ref": "functional-additions.schema.json#/$defs/FSLinearCombineFieldsInput"
      },
      "Math_FSCartesianLineInput": {
        "$ref": "functional-additions.schema.json#/$defs/FSCartesianLineInput"
      },
      "Math_FSCartesianLineOutput": {
        "$ref": "functional-additions.schema.json#/$defs/FSCartesianLineOutput"
      },
      "Math_FSLinearCombineCurvesInput": {
        "$ref": "functional-additions.schema.json#/$defs/FSLinearCombineCurvesInput"
      },
      "Math_FSLinearCombineCurvesOutput": {
        "$ref": "functional-additions.schema.json#/$defs/FSLinearCombineCurvesOutput"
      },
      "F32Bits": {
        "type": "string",
        "pattern": "^[0-9a-f]{8}$"
      },
      "F64Bits": {
        "type": "string",
        "pattern": "^[0-9a-f]{16}$"
      },
      "ComplexF64Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/F64Bits"
        },
        "minItems": 2,
        "maxItems": 2
      },
      "Vec3F32Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/F32Bits"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Vec3F64Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/F64Bits"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Mat4ColumnsF64Bits": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/F64Bits"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "minItems": 4,
        "maxItems": 4
      },
      "TableSourceIdentity": {
        "type": "object",
        "properties": {
          "resourceDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "sourceId": {
            "type": "string",
            "minLength": 1
          },
          "sourceVersion": {
            "type": "string",
            "minLength": 1
          },
          "edition": {
            "type": "string",
            "minLength": 1
          },
          "tableKey": {
            "type": "string",
            "minLength": 1
          },
          "tableContentDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "producerKind": {
            "enum": [
              "shipped-original",
              "author-primary-document",
              "caller-explicit-data",
              "authored-mathematical-prior"
            ]
          },
          "producerName": {
            "type": "string",
            "minLength": 1
          },
          "consumerRole": {
            "enum": [
              "native-group-setting",
              "native-gridno",
              "native-scattering",
              "native-powder",
              "native-isoview",
              "explicit-math"
            ]
          },
          "licenseDisposition": {
            "enum": [
              "deferred-current-user-decision",
              "not-applicable-own-input"
            ]
          },
          "licenseTextDigest": {
            "oneOf": [
              {
                "$ref": "#/$defs/FS1_Digest"
              },
              {
                "type": "null"
              }
            ]
          },
          "writtenPermissionEvidenceDigest": {
            "oneOf": [
              {
                "$ref": "#/$defs/FS1_Digest"
              },
              {
                "type": "null"
              }
            ]
          },
          "grantsRedistribution": {
            "const": false
          }
        },
        "required": [
          "resourceDigest",
          "sourceId",
          "sourceVersion",
          "edition",
          "tableKey",
          "tableContentDigest",
          "producerKind",
          "producerName",
          "consumerRole",
          "licenseDisposition",
          "licenseTextDigest",
          "writtenPermissionEvidenceDigest",
          "grantsRedistribution"
        ],
        "additionalProperties": false
      },
      "ScientificTableUse": {
        "type": "object",
        "properties": {
          "identity": {
            "$ref": "#/$defs/TableSourceIdentity"
          },
          "species": {
            "$ref": "#/$defs/FS1_Species"
          },
          "purpose": {
            "enum": [
              "xray-f0",
              "anomalous",
              "neutron-bc",
              "isotope-mass",
              "constant",
              "group-setting",
              "wyckoff"
            ]
          },
          "rowSource": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "scientificQualificationRecordDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "identity",
          "species",
          "purpose",
          "rowSource",
          "scientificQualificationRecordDigest"
        ],
        "additionalProperties": false
      },
      "NativeSettingBinding": {
        "type": "object",
        "properties": {
          "groupNumberOneBased": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "settingNumberOneBased": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "registryTable": {
            "$ref": "#/$defs/TableSourceIdentity"
          },
          "registryRecord": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "completeWyckoffPopulationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "completeCurrentSymmetryPopulationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "qualificationRecordDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "groupNumberOneBased",
          "settingNumberOneBased",
          "registryTable",
          "registryRecord",
          "completeWyckoffPopulationDigest",
          "completeCurrentSymmetryPopulationDigest",
          "qualificationRecordDigest"
        ],
        "additionalProperties": false
      },
      "WyckoffAffine": {
        "type": "object",
        "properties": {
          "sourceOrdinalOneBased": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "affineColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          }
        },
        "required": [
          "sourceOrdinalOneBased",
          "affineColumnsF64Bits",
          "source"
        ],
        "additionalProperties": false
      },
      "CurrentSymmetryMatrix": {
        "type": "object",
        "properties": {
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "affineColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "qualificationRecordDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "sourceOrdinalZeroBased",
          "affineColumnsF64Bits",
          "qualificationRecordDigest"
        ],
        "additionalProperties": false
      },
      "NativeGridNoInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-gridno"
          },
          "dimension": {
            "const": 3
          },
          "axisLengthsF32Bits": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "resolutionF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "setting": {
            "$ref": "#/$defs/NativeSettingBinding"
          },
          "currentBasisColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "wyckoffPopulation": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/WyckoffAffine"
            },
            "minItems": 0
          },
          "currentSymmetryPopulation": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/CurrentSymmetryMatrix"
            },
            "minItems": 0
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "dimension",
          "axisLengthsF32Bits",
          "resolutionF32Bits",
          "setting",
          "currentBasisColumnsF64Bits",
          "wyckoffPopulation",
          "currentSymmetryPopulation",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeGridNoOutput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-gridno"
          },
          "requiredAxisMultiples": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "divisionsBeforeFallback": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "coreDivisions": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "reflectionAxisBounds": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "sourceSetting": {
            "$ref": "#/$defs/NativeSettingBinding"
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "requiredAxisMultiples",
          "divisionsBeforeFallback",
          "coreDivisions",
          "reflectionAxisBounds",
          "parameterDigest",
          "sourceSetting"
        ],
        "additionalProperties": false
      },
      "AmplitudeUnitBinding": {
        "type": "object",
        "properties": {
          "unit": {
            "enum": [
              "electron",
              "fm"
            ]
          },
          "physicalQualificationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "tableUses": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificTableUse"
            },
            "minItems": 0
          }
        },
        "required": [
          "unit",
          "physicalQualificationDigest",
          "tableUses"
        ],
        "additionalProperties": false
      },
      "EquivalentReflection": {
        "type": "object",
        "properties": {
          "hkl": {
            "$ref": "#/$defs/FS1_Int3"
          },
          "phasePairF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hkl",
          "phasePairF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "MaterializedReflection": {
        "type": "object",
        "properties": {
          "representativeHkl": {
            "$ref": "#/$defs/FS1_Int3"
          },
          "physicalReciprocalMagnitudePerAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "equivalents": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/EquivalentReflection"
            },
            "minItems": 1
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "representativeHkl",
          "physicalReciprocalMagnitudePerAF64Bits",
          "coefficientF64Bits",
          "equivalents",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "ReflectionSource": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "sourceFormat": {
            "enum": [
              "native-fcf",
              "native-cif",
              "native-mem",
              "native-fos",
              "native-hkl",
              "native-out",
              "native-free-format",
              "authored-explicit-complex"
            ]
          },
          "readerSelection": {
            "type": "integer",
            "minimum": 0,
            "maximum": 3
          },
          "materializedRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MaterializedReflection"
            },
            "minItems": 0
          },
          "actualF000AfterReadF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "amplitude": {
            "$ref": "#/$defs/AmplitudeUnitBinding"
          },
          "readerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "materializedCurrentDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "base",
          "phaseId",
          "source",
          "sourceFormat",
          "readerSelection",
          "materializedRows",
          "actualF000AfterReadF64Bits",
          "amplitude",
          "readerRuleDigest",
          "materializedCurrentDigest"
        ],
        "additionalProperties": false
      },
      "FourierUi0Input": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-fourier-ui"
          },
          "dimension": {
            "const": 3
          },
          "uiSelection": {
            "const": 0
          },
          "uiLabel": {
            "const": "Fo - Fc"
          },
          "firstSource": {
            "$ref": "#/$defs/ReflectionSource"
          },
          "actualCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "f000Policy": {
            "const": "use-actual-current-scalar-separate-from-difference"
          },
          "grid": {
            "$ref": "#/$defs/NativeGridNoInput"
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "dimension",
          "uiSelection",
          "uiLabel",
          "firstSource",
          "actualCurrentF000F64Bits",
          "f000Policy",
          "grid"
        ],
        "additionalProperties": false
      },
      "FourierUi1Input": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-fourier-ui"
          },
          "dimension": {
            "const": 3
          },
          "uiSelection": {
            "const": 1
          },
          "uiLabel": {
            "const": "Fo"
          },
          "firstSource": {
            "$ref": "#/$defs/ReflectionSource"
          },
          "actualCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "f000Policy": {
            "const": "use-actual-current-scalar-separate-from-difference"
          },
          "grid": {
            "$ref": "#/$defs/NativeGridNoInput"
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "dimension",
          "uiSelection",
          "uiLabel",
          "firstSource",
          "actualCurrentF000F64Bits",
          "f000Policy",
          "grid"
        ],
        "additionalProperties": false
      },
      "FourierUi2Input": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-fourier-ui"
          },
          "dimension": {
            "const": 3
          },
          "uiSelection": {
            "const": 2
          },
          "uiLabel": {
            "const": "Fc"
          },
          "firstSource": {
            "$ref": "#/$defs/ReflectionSource"
          },
          "actualCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "f000Policy": {
            "const": "use-actual-current-scalar-separate-from-difference"
          },
          "grid": {
            "$ref": "#/$defs/NativeGridNoInput"
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "dimension",
          "uiSelection",
          "uiLabel",
          "firstSource",
          "actualCurrentF000F64Bits",
          "f000Policy",
          "grid"
        ],
        "additionalProperties": false
      },
      "FourierUi3Input": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-fourier-ui"
          },
          "dimension": {
            "const": 3
          },
          "uiSelection": {
            "const": 3
          },
          "uiLabel": {
            "const": "Fo - Fo(another data)"
          },
          "firstSource": {
            "$ref": "#/$defs/ReflectionSource"
          },
          "actualCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "f000Policy": {
            "const": "use-actual-current-scalar-separate-from-difference"
          },
          "grid": {
            "$ref": "#/$defs/NativeGridNoInput"
          },
          "secondSource": {
            "$ref": "#/$defs/ReflectionSource"
          },
          "secondScaleF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "differencePolicy": {
            "const": "native-spacing-window-phase-aligned-asymmetric-missing-rule"
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "dimension",
          "uiSelection",
          "uiLabel",
          "firstSource",
          "actualCurrentF000F64Bits",
          "f000Policy",
          "grid",
          "secondSource",
          "secondScaleF64Bits",
          "differencePolicy"
        ],
        "additionalProperties": false
      },
      "NativeFourierUiInput": {
        "oneOf": [
          {
            "$ref": "#/$defs/FourierUi0Input"
          },
          {
            "$ref": "#/$defs/FourierUi1Input"
          },
          {
            "$ref": "#/$defs/FourierUi2Input"
          },
          {
            "$ref": "#/$defs/FourierUi3Input"
          }
        ]
      },
      "FourierSourceDiagnostic": {
        "type": "object",
        "properties": {
          "code": {
            "enum": [
              "extinction-reset-zero",
              "unmatched-retained",
              "unmatched-removed",
              "duplicate-removed",
              "native-nonfinite",
              "parse-error"
            ]
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "hkl": {
            "$ref": "#/$defs/FS1_Int3"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          }
        },
        "required": [
          "code",
          "sourceOrdinalZeroBased",
          "hkl",
          "source"
        ],
        "additionalProperties": false
      },
      "NativePreparedFourierOutput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "uiSelection": {
            "type": "integer",
            "minimum": 0,
            "maximum": 3
          },
          "rawBackendMode": {
            "const": 0
          },
          "actualCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "rows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MaterializedReflection"
            },
            "minItems": 0
          },
          "sourceBindings": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ReflectionSource"
            },
            "minItems": 1,
            "maxItems": 2
          },
          "secondScaleF64Bits": {
            "oneOf": [
              {
                "$ref": "#/$defs/F64Bits"
              },
              {
                "type": "null"
              }
            ]
          },
          "grid": {
            "$ref": "#/$defs/NativeGridNoOutput"
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FourierSourceDiagnostic"
            },
            "minItems": 0
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "base",
          "phaseId",
          "uiSelection",
          "rawBackendMode",
          "actualCurrentF000F64Bits",
          "rows",
          "sourceBindings",
          "secondScaleF64Bits",
          "grid",
          "diagnostics",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "PowderTch13Parameters": {
        "type": "object",
        "properties": {
          "nameCount": {
            "const": 13
          },
          "fitParameterCount": {
            "const": 10
          },
          "parameterNames": {
            "type": "array",
            "prefixItems": [
              {
                "const": "U"
              },
              {
                "const": "V"
              },
              {
                "const": "W"
              },
              {
                "const": "P"
              },
              {
                "const": "X"
              },
              {
                "const": "Xe"
              },
              {
                "const": "Y"
              },
              {
                "const": "Ye"
              },
              {
                "const": "rs"
              },
              {
                "const": "rd"
              },
              {
                "const": "h"
              },
              {
                "const": "k"
              },
              {
                "const": "l"
              }
            ],
            "items": false,
            "minItems": 13,
            "maxItems": 13
          },
          "U": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegreeSquared"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x20"
          },
          "V": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegreeSquared"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x28"
          },
          "W": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegreeSquared"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x30"
          },
          "P": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegreeSquared"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x38"
          },
          "X": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegree"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x40"
          },
          "Xe": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegree"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x48"
          },
          "Y": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegree"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x50"
          },
          "Ye": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "twoThetaDegree"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x58"
          },
          "rs": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "dimensionlessNativeProfile"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x60"
          },
          "rd": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "dimensionlessNativeProfile"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x68"
          },
          "h": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "dimensionlessNativeProfile"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x70"
          },
          "k": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "dimensionlessNativeProfile"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x78"
          },
          "l": {
            "type": "object",
            "properties": {
              "valueF64Bits": {
                "$ref": "#/$defs/F64Bits"
              },
              "unit": {
                "const": "dimensionlessNativeProfile"
              }
            },
            "required": [
              "valueF64Bits",
              "unit"
            ],
            "additionalProperties": false,
            "x-nativeOffset": "0x80"
          },
          "parameterTableSource": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "parameterRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "nameCount",
          "fitParameterCount",
          "parameterNames",
          "U",
          "V",
          "W",
          "P",
          "X",
          "Xe",
          "Y",
          "Ye",
          "rs",
          "rd",
          "h",
          "k",
          "l",
          "parameterTableSource",
          "parameterRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeTchWidthInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-tch-width"
          },
          "parameters": {
            "$ref": "#/$defs/PowderTch13Parameters"
          },
          "twoThetaDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "reflectionRawAxisScalarF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "rawAxisScalarRule": {
            "const": "CalcCosPhi-then-native-cos-of-raw-times-pi-over360"
          },
          "currentReciprocalMatrixColumnsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "parameters",
          "twoThetaDegreeF64Bits",
          "reflectionRawAxisScalarF64Bits",
          "rawAxisScalarRule",
          "currentReciprocalMatrixColumnsF64Bits"
        ],
        "additionalProperties": false
      },
      "NativeTchWidthOutput": {
        "type": "object",
        "properties": {
          "profile": {
            "const": "vesta3909a-native-tch-width"
          },
          "gaussianWidthDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "lorentzWidthDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "combinedWidthDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "mixingF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "profile",
          "gaussianWidthDegreeF64Bits",
          "lorentzWidthDegreeF64Bits",
          "combinedWidthDegreeF64Bits",
          "mixingF64Bits",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativePowderReflectionInput": {
        "type": "object",
        "properties": {
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "coefficientUnit": {
            "enum": [
              "electron",
              "fm",
              "native-source-amplitude-unqualified"
            ]
          },
          "multiplicity": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "wavelengthAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "beamWeightF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "reflectionCorrectionF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "perObservationFactorsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "materializedProfileF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "profileUnit": {
            "const": "nativeProfilePerTwoThetaDegree"
          },
          "profileProducer": {
            "enum": [
              "vesta3909a-TCH",
              "vesta3909a-TCH-Ida",
              "authored-explicit-control"
            ]
          },
          "profileRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "materializedProfileStage": {
            "const": "unweighted-line-shape-before-observation-beam-phase"
          }
        },
        "required": [
          "coefficientF64Bits",
          "coefficientUnit",
          "multiplicity",
          "wavelengthAF64Bits",
          "beamWeightF64Bits",
          "reflectionCorrectionF64Bits",
          "perObservationFactorsF64Bits",
          "materializedProfileF64Bits",
          "profileUnit",
          "profileProducer",
          "profileRuleDigest",
          "source",
          "materializedProfileStage"
        ],
        "additionalProperties": false
      },
      "NativePowderPhaseInput": {
        "type": "object",
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "phaseScaleF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "reflections": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePowderReflectionInput"
            },
            "minItems": 0
          },
          "scientificTableUses": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificTableUse"
            },
            "minItems": 0
          }
        },
        "required": [
          "phaseId",
          "phaseScaleF64Bits",
          "reflections",
          "scientificTableUses"
        ],
        "additionalProperties": false
      },
      "NativePowderNormalization": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "no-observed-pattern"
              },
              "target": {
                "const": 100
              }
            },
            "required": [
              "kind",
              "target"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "actual-observed-pattern"
              },
              "observed": {
                "$ref": "#/$defs/FS1_Curve"
              },
              "actualStoredObservedMaxF64Bits": {
                "$ref": "#/$defs/F64Bits"
              }
            },
            "required": [
              "kind",
              "observed",
              "actualStoredObservedMaxF64Bits"
            ],
            "additionalProperties": false
          }
        ]
      },
      "NativePowderAccumulateInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "profile": {
            "const": "vesta3909a-native-pattern-accumulation"
          },
          "sampleTwoThetaDegreeF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "phases": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePowderPhaseInput"
            },
            "minItems": 0
          },
          "backgroundF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "normalization": {
            "$ref": "#/$defs/NativePowderNormalization"
          },
          "policy": {
            "const": "sum-all-phases-normalize-shared-maximum-then-add-background"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "profile",
          "sampleTwoThetaDegreeF64Bits",
          "phases",
          "backgroundF64Bits",
          "normalization",
          "policy",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativePowderAccumulateOutput": {
        "type": "object",
        "properties": {
          "profile": {
            "const": "vesta3909a-native-pattern-accumulation"
          },
          "sampleTwoThetaDegreeF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "rawPhaseCurvesF64Bits": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/F64Bits"
              },
              "minItems": 0
            },
            "minItems": 0
          },
          "rawAggregateF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "maxRawYcF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "targetF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "commonScaleF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "scaledPhaseCurvesF64Bits": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/F64Bits"
              },
              "minItems": 0
            },
            "minItems": 0
          },
          "backgroundF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "totalF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "unit": {
            "const": "relativeDisplayIntensity"
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "profile",
          "sampleTwoThetaDegreeF64Bits",
          "rawPhaseCurvesF64Bits",
          "rawAggregateF64Bits",
          "maxRawYcF64Bits",
          "targetF64Bits",
          "commonScaleF64Bits",
          "scaledPhaseCurvesF64Bits",
          "backgroundF64Bits",
          "totalF64Bits",
          "unit",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "QualifiedFieldSource": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "qualified-fs-field"
          },
          "field": {
            "$ref": "#/$defs/FS1_FSFieldBinding"
          },
          "sourceFormatDescriptorDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "kind",
          "field",
          "sourceFormatDescriptorDigest"
        ],
        "additionalProperties": false
      },
      "NativeFractionalSource": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "native-wien-current-single-f32"
          },
          "current": {
            "$ref": "#/$defs/FC_NativeFractionalGridCurrent1"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "kind",
          "current",
          "source",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeAffineSource": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "native-affine-current-scalar"
          },
          "current": {
            "$ref": "#/$defs/FC_NativeScalarGridCurrent1"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "kind",
          "current",
          "source",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "ScalarSource": {
        "oneOf": [
          {
            "$ref": "#/$defs/QualifiedFieldSource"
          },
          {
            "$ref": "#/$defs/NativeFractionalSource"
          },
          {
            "$ref": "#/$defs/NativeAffineSource"
          }
        ]
      },
      "NoComponentSelector": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "none"
          }
        },
        "required": [
          "kind"
        ],
        "additionalProperties": false
      },
      "FlatIgnoredSelector": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "native-grid30-flat-selector-ignored"
          },
          "selectionIndexI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "group0Count": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "group1Count": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "remapFlagI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          }
        },
        "required": [
          "kind",
          "selectionIndexI32",
          "group0Count",
          "group1Count",
          "remapFlagI32"
        ],
        "additionalProperties": false
      },
      "IndexedNativeSelector": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "native-grid30-indexed-selector"
          },
          "selectionIndexI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "group0Count": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "group1Count": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "remapFlagI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "secondGroupRule": {
            "const": "selection-equals-two-times-n0-plus-localOrdinal1"
          }
        },
        "required": [
          "kind",
          "selectionIndexI32",
          "group0Count",
          "group1Count",
          "remapFlagI32",
          "secondGroupRule"
        ],
        "additionalProperties": false
      },
      "IndexedCorrectedSelector": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "corrected-grid30-indexed-selector"
          },
          "selectionIndexI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "group0Count": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "group1Count": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "remapFlagI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "secondGroupRule": {
            "const": "selection-equals-n0-plus-localOrdinal1"
          }
        },
        "required": [
          "kind",
          "selectionIndexI32",
          "group0Count",
          "group1Count",
          "remapFlagI32",
          "secondGroupRule"
        ],
        "additionalProperties": false
      },
      "ComponentSelector": {
        "oneOf": [
          {
            "$ref": "#/$defs/NoComponentSelector"
          },
          {
            "$ref": "#/$defs/FlatIgnoredSelector"
          },
          {
            "$ref": "#/$defs/IndexedNativeSelector"
          },
          {
            "$ref": "#/$defs/IndexedCorrectedSelector"
          }
        ]
      },
      "OrderedExpressionEntry": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "originalReference": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "sourceFormat": {
            "enum": [
              "materialized-qualified-field",
              "native-wien-single",
              "native-grid30-flat",
              "native-grid30-indexed",
              "corrected-grid30-indexed"
            ]
          },
          "selector": {
            "$ref": "#/$defs/ComponentSelector"
          },
          "prefix": {
            "enum": [
              "add",
              "multiply",
              "divide"
            ]
          },
          "coefficientF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "powerF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "formatRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "source",
          "originalReference",
          "sourceFormat",
          "selector",
          "prefix",
          "coefficientF32Bits",
          "powerF32Bits",
          "formatRuleDigest"
        ],
        "additionalProperties": false
      },
      "ResultQuantityBinding": {
        "type": "object",
        "properties": {
          "meaning": {
            "enum": [
              "qualified-physical",
              "uninterpreted-current-numerical"
            ]
          },
          "physicalKind": {
            "enum": [
              "electronDensity",
              "nuclearDensity",
              "electronPatterson",
              "nuclearPatterson",
              "generic"
            ]
          },
          "unit": {
            "enum": [
              "ePerA3",
              "fmPerA3",
              "e2PerA3",
              "fm2PerA3",
              "declared"
            ]
          },
          "declaredUnit": {
            "oneOf": [
              {
                "type": "string",
                "minLength": 1
              },
              {
                "type": "null"
              }
            ]
          },
          "resultQuantityRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "meaning",
          "physicalKind",
          "unit",
          "declaredUnit",
          "resultQuantityRuleDigest"
        ],
        "additionalProperties": false
      },
      "OrderedExpressionInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "enum": [
              "vesta3909a-native-f32-ordered-expression",
              "corrected-explicit-f32-ordered-expression"
            ]
          },
          "entries": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/OrderedExpressionEntry"
            },
            "minItems": 1
          },
          "initialization": {
            "const": "empty-new-output"
          },
          "geometryPolicy": {
            "const": "require-exact-qualified-common-geometry"
          },
          "resultQuantity": {
            "$ref": "#/$defs/ResultQuantityBinding"
          },
          "arithmetic": {
            "const": "native-first-mul32-add-fma32-multiply-mul32-divide-div32-then-mul32"
          },
          "reopen": {
            "const": "restore-materialized-current-do-not-replay"
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "entries",
          "initialization",
          "geometryPolicy",
          "resultQuantity",
          "arithmetic",
          "reopen",
          "maxResidentBytes",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "OrderedExpressionOutput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "materialized": {
            "$ref": "#/$defs/ScalarSource"
          },
          "ownership": {
            "const": "candidate-retained"
          },
          "orderedSources": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/OrderedExpressionEntry"
            },
            "minItems": 1
          },
          "resultQuantity": {
            "$ref": "#/$defs/ResultQuantityBinding"
          },
          "materializedDataDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "materializedDescriptorDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "operationRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "base",
          "phaseId",
          "materialized",
          "ownership",
          "orderedSources",
          "resultQuantity",
          "materializedDataDigest",
          "materializedDescriptorDigest",
          "operationRuleDigest",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "ExactComparison": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "exact"
          },
          "domain": {
            "enum": [
              "discrete",
              "ieee754-bits",
              "rational",
              "symbolic-rational-plus-coefficient-over-pi"
            ]
          }
        },
        "required": [
          "kind",
          "domain"
        ],
        "additionalProperties": false
      },
      "BoundedComparison": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "fixture-f64-absolute-bound"
          },
          "absoluteBound": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "unit": {
            "type": "string",
            "minLength": 1
          },
          "scope": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "kind",
          "absoluteBound",
          "unit",
          "scope"
        ],
        "additionalProperties": false
      },
      "OracleBinding": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "classification": {
            "enum": [
              "original-frozen-prior",
              "independent-mathematical-prior",
              "source-derived-reference",
              "supporting-only"
            ]
          },
          "expectedPath": {
            "type": "string",
            "minLength": 1
          },
          "expectedSha256": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "expectedPointers": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            },
            "minItems": 1
          },
          "requiredOutputFields": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            },
            "minItems": 1
          },
          "comparison": {
            "oneOf": [
              {
                "$ref": "#/$defs/ExactComparison"
              },
              {
                "$ref": "#/$defs/BoundedComparison"
              }
            ]
          },
          "completeArrayCounts": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          },
          "nativeExecutionClaim": {
            "const": false
          }
        },
        "required": [
          "id",
          "classification",
          "expectedPath",
          "expectedSha256",
          "expectedPointers",
          "requiredOutputFields",
          "comparison",
          "completeArrayCounts",
          "nativeExecutionClaim"
        ],
        "additionalProperties": false
      },
      "SG2_NativeDimensionDState": {
        "$ref": "../sg/sg2.schema.json#/$defs/NativeDimensionDState"
      },
      "FC_NativeLayerCurrent1": {
        "$ref": "../../../../../20261115-092025-p1-03-completion/native-layer-current.schema.json"
      },
      "FS1_Mesh": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Mesh"
      },
      "FS1_FSNonfiniteDiagnostic": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSNonfiniteDiagnostic"
      },
      "FS1_FSConversionResult": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSConversionResult"
      },
      "FS1_FSPeakResult": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSPeakResult"
      },
      "FS1_Field": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Field"
      },
      "FS1_Plane": {
        "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Plane"
      },
      "NativeFailure": {
        "type": "object",
        "properties": {
          "status": {
            "const": "reference-failure"
          },
          "code": {
            "enum": [
              "E_REFERENCE_FAILURE",
              "E_NATIVE_NONFINITE",
              "E_NATIVE_GRID_DIMENSION_LAYOUT",
              "E_SOURCE_PARSE",
              "E_SOURCE_BOUNDS",
              "E_ZERO_PROFILE_MAX"
            ]
          },
          "sourceProcedure": {
            "type": "string",
            "minLength": 1
          },
          "sourceAddress": {
            "type": "string",
            "minLength": 1
          },
          "diagnostic": {
            "type": "string",
            "minLength": 1
          },
          "preservedInputDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "candidateReleased": {
            "const": true
          }
        },
        "required": [
          "status",
          "code",
          "sourceProcedure",
          "sourceAddress",
          "diagnostic",
          "preservedInputDigest",
          "candidateReleased"
        ],
        "additionalProperties": false
      },
      "ScientificSourceRow": {
        "type": "object",
        "properties": {
          "speciesLabel": {
            "type": "string",
            "minLength": 1
          },
          "atomicNumberI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "actualElementIndexI16": {
            "type": "integer",
            "minimum": -32768,
            "maximum": 32767
          },
          "asfdcSource": {
            "$ref": "#/$defs/TableSourceIdentity"
          },
          "selectedFourLineSpan": {
            "oneOf": [
              {
                "$ref": "#/$defs/FS1_FSByteSpan"
              },
              {
                "type": "null"
              }
            ]
          },
          "selection": {
            "enum": [
              "exact-complete-label",
              "last-provisional-source-row",
              "no-matching-source-row-zero-amplitudes"
            ]
          },
          "candidateSpansInActualScanOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS1_FSByteSpan"
            },
            "minItems": 0
          },
          "fiveGaussianAF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 5,
            "maxItems": 5
          },
          "fiveGaussianBA2F64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 5,
            "maxItems": 5
          },
          "constantElectronF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "coherentBcFmF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "printedMassF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "printedMassUnit": {
            "const": "source-printed-relative-atomic-mass"
          }
        },
        "required": [
          "speciesLabel",
          "atomicNumberI32",
          "actualElementIndexI16",
          "asfdcSource",
          "selectedFourLineSpan",
          "selection",
          "candidateSpansInActualScanOrder",
          "fiveGaussianAF64Bits",
          "fiveGaussianBA2F64Bits",
          "constantElectronF64Bits",
          "coherentBcFmF64Bits",
          "printedMassF64Bits",
          "sourceRuleDigest",
          "printedMassUnit"
        ],
        "additionalProperties": false
      },
      "DispersionEightColumnRow": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "valuesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 8,
            "maxItems": 8
          }
        },
        "required": [
          "source",
          "valuesF32Bits"
        ],
        "additionalProperties": false
      },
      "NativeDispersionInput": {
        "type": "object",
        "properties": {
          "profile": {
            "const": "vesta3909a-ffast-linear-f32"
          },
          "sourceIdentity": {
            "$ref": "#/$defs/TableSourceIdentity"
          },
          "atomicNumberI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "wavelengthNmF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "relativisticCorrectionF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "nuclearThomsonF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "completeRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/DispersionEightColumnRow"
            },
            "minItems": 1
          },
          "sourceEdgeHeaderSpans": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS1_FSByteSpan"
            },
            "minItems": 0
          }
        },
        "required": [
          "profile",
          "sourceIdentity",
          "atomicNumberI32",
          "wavelengthNmF32Bits",
          "relativisticCorrectionF32Bits",
          "nuclearThomsonF32Bits",
          "completeRows",
          "sourceEdgeHeaderSpans"
        ],
        "additionalProperties": false
      },
      "NativeDispersionOutput": {
        "type": "object",
        "properties": {
          "energyKeVF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "lowerSourceOrdinal": {
            "oneOf": [
              {
                "type": "integer",
                "minimum": 0,
                "maximum": 9007199254740991
              },
              {
                "type": "null"
              }
            ]
          },
          "upperSourceOrdinal": {
            "oneOf": [
              {
                "type": "integer",
                "minimum": 0,
                "maximum": 9007199254740991
              },
              {
                "type": "null"
              }
            ]
          },
          "lowerWasInitializedZero": {
            "type": "boolean"
          },
          "upperWasResetZeroAtEof": {
            "type": "boolean"
          },
          "fpElectronF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "fppElectronF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "massAttenuationCm2PerGF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "energyKeVF32Bits",
          "lowerSourceOrdinal",
          "upperSourceOrdinal",
          "lowerWasInitializedZero",
          "upperWasResetZeroAtEof",
          "fpElectronF32Bits",
          "fppElectronF32Bits",
          "massAttenuationCm2PerGF32Bits",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeEquivalentD3": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "phasePairF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "phasePairF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionD3": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "physicalReciprocalMagnitudePerAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "internalReciprocalMagnitudeNativeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "integerPhaseMultiplicityI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "friedelFlagsU32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "equivalents": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeEquivalentD3"
            },
            "minItems": 1
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "physicalReciprocalMagnitudePerAF64Bits",
          "internalReciprocalMagnitudeNativeF64Bits",
          "integerPhaseMultiplicityI32",
          "friedelFlagsU32",
          "equivalents",
          "coefficientF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionPopulationD3": {
        "type": "object",
        "properties": {
          "dimensionD": {
            "const": 3
          },
          "rows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeReflectionD3"
            },
            "minItems": 0
          },
          "populationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "producerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "dimensionD",
          "rows",
          "populationDigest",
          "producerRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeEquivalentD4": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 4,
            "maxItems": 4
          },
          "phasePairF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "phasePairF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionD4": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 4,
            "maxItems": 4
          },
          "physicalReciprocalMagnitudePerAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "internalReciprocalMagnitudeNativeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "integerPhaseMultiplicityI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "friedelFlagsU32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "equivalents": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeEquivalentD4"
            },
            "minItems": 1
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "physicalReciprocalMagnitudePerAF64Bits",
          "internalReciprocalMagnitudeNativeF64Bits",
          "integerPhaseMultiplicityI32",
          "friedelFlagsU32",
          "equivalents",
          "coefficientF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionPopulationD4": {
        "type": "object",
        "properties": {
          "dimensionD": {
            "const": 4
          },
          "rows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeReflectionD4"
            },
            "minItems": 0
          },
          "populationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "producerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "dimensionD",
          "rows",
          "populationDigest",
          "producerRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeEquivalentD5": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 5,
            "maxItems": 5
          },
          "phasePairF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "phasePairF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionD5": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 5,
            "maxItems": 5
          },
          "physicalReciprocalMagnitudePerAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "internalReciprocalMagnitudeNativeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "integerPhaseMultiplicityI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "friedelFlagsU32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "equivalents": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeEquivalentD5"
            },
            "minItems": 1
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "physicalReciprocalMagnitudePerAF64Bits",
          "internalReciprocalMagnitudeNativeF64Bits",
          "integerPhaseMultiplicityI32",
          "friedelFlagsU32",
          "equivalents",
          "coefficientF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionPopulationD5": {
        "type": "object",
        "properties": {
          "dimensionD": {
            "const": 5
          },
          "rows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeReflectionD5"
            },
            "minItems": 0
          },
          "populationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "producerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "dimensionD",
          "rows",
          "populationDigest",
          "producerRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeEquivalentD6": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 6,
            "maxItems": 6
          },
          "phasePairF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "phasePairF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionD6": {
        "type": "object",
        "properties": {
          "hklD": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 6,
            "maxItems": 6
          },
          "physicalReciprocalMagnitudePerAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "internalReciprocalMagnitudeNativeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "integerPhaseMultiplicityI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "friedelFlagsU32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "equivalents": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeEquivalentD6"
            },
            "minItems": 1
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "sourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "hklD",
          "physicalReciprocalMagnitudePerAF64Bits",
          "internalReciprocalMagnitudeNativeF64Bits",
          "integerPhaseMultiplicityI32",
          "friedelFlagsU32",
          "equivalents",
          "coefficientF64Bits",
          "sourceOrdinalZeroBased"
        ],
        "additionalProperties": false
      },
      "NativeReflectionPopulationD6": {
        "type": "object",
        "properties": {
          "dimensionD": {
            "const": 6
          },
          "rows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeReflectionD6"
            },
            "minItems": 0
          },
          "populationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "producerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "dimensionD",
          "rows",
          "populationDigest",
          "producerRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeReflectionPopulation": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeReflectionPopulationD3"
          },
          {
            "$ref": "#/$defs/NativeReflectionPopulationD4"
          },
          {
            "$ref": "#/$defs/NativeReflectionPopulationD5"
          },
          {
            "$ref": "#/$defs/NativeReflectionPopulationD6"
          }
        ]
      },
      "NativeThermalSelection": {
        "type": "object",
        "properties": {
          "anisotropicEnabledI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "anisotropicRepresentation": {
            "enum": [
              "phase214-Uij0",
              "phase214-beta1"
            ]
          },
          "isotropicRepresentation": {
            "enum": [
              "phase218-U0",
              "phase218-B1"
            ]
          },
          "perSiteFlagsPolicy": {
            "const": "actual-da-bit3-source-dispatch"
          },
          "covariancePolicy": {
            "const": "native-diagonal-and-actual-SUD; ORFFE-full-is-separate-explicit-profile"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "anisotropicEnabledI32",
          "anisotropicRepresentation",
          "isotropicRepresentation",
          "perSiteFlagsPolicy",
          "covariancePolicy",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeScatteringInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-actual-site-scattering"
          },
          "model": {
            "$ref": "#/$defs/SG2_NativeDimensionDState"
          },
          "radiation": {
            "enum": [
              "xray-electron-amplitude",
              "neutron-coherent-fm"
            ]
          },
          "thermal": {
            "$ref": "#/$defs/NativeThermalSelection"
          },
          "completeElementRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          },
          "completeDispersionByElement": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeDispersionOutput"
            },
            "minItems": 0
          },
          "actualSymmetryOperatorCount": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "reflections": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "model",
          "radiation",
          "thermal",
          "completeElementRows",
          "completeDispersionByElement",
          "actualSymmetryOperatorCount",
          "reflections",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "SiteReflectionContribution": {
        "type": "object",
        "properties": {
          "siteId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "reflectionSourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "equivalentSourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "thermalFactorF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "weightedAmplitudeF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          }
        },
        "required": [
          "siteId",
          "reflectionSourceOrdinalZeroBased",
          "equivalentSourceOrdinalZeroBased",
          "coefficientF64Bits",
          "thermalFactorF64Bits",
          "weightedAmplitudeF64Bits"
        ],
        "additionalProperties": false
      },
      "NativeScatteringOutput": {
        "type": "object",
        "properties": {
          "profile": {
            "const": "vesta3909a-actual-site-scattering"
          },
          "reflections": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "allSiteEquivalentContributions": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SiteReflectionContribution"
            },
            "minItems": 0
          },
          "amplitudeUnit": {
            "enum": [
              "electron",
              "fm"
            ]
          },
          "elementRowsUsed": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS1_Diagnostic"
            },
            "minItems": 0
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "profile",
          "reflections",
          "allSiteEquivalentContributions",
          "amplitudeUnit",
          "elementRowsUsed",
          "diagnostics",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeFieldGrid": {
        "type": "object",
        "properties": {
          "actualDimensionD": {
            "enum": [
              3,
              4,
              5,
              6
            ]
          },
          "nativeThreeDivisions": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "currentC2OColumnsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "physicalFieldQuantity": {
            "enum": [
              "electronDensity",
              "nuclearDensity",
              "electronPatterson",
              "nuclearPatterson"
            ]
          },
          "sourceLayoutRule": {
            "const": "VolData-c-10-14-divisions-18-vector-pointer"
          },
          "gridProducerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "actualDimensionD",
          "nativeThreeDivisions",
          "currentC2OColumnsF64Bits",
          "physicalFieldQuantity",
          "sourceLayoutRule",
          "gridProducerRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeFourierFieldInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-full-fourier"
          },
          "grid": {
            "$ref": "#/$defs/NativeFieldGrid"
          },
          "reflections": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "actualCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "rawModeI32": {
            "enum": [
              0,
              1,
              2,
              3
            ]
          },
          "amplitudeUnit": {
            "enum": [
              "electron",
              "fm",
              "electron2",
              "fm2"
            ]
          },
          "packingPolicy": {
            "const": "native-half-grid-last-writer-flags-and-Friedel-order"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "grid",
          "reflections",
          "actualCurrentF000F64Bits",
          "rawModeI32",
          "amplitudeUnit",
          "packingPolicy",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeFullFieldOutput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "grid": {
            "$ref": "#/$defs/NativeFieldGrid"
          },
          "coreValuesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 0
          },
          "periodicClosureValuesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 0
          },
          "packedSpectrumF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ComplexF64Bits"
            },
            "minItems": 0
          },
          "field": {
            "$ref": "#/$defs/ScalarSource"
          },
          "quantityUnit": {
            "enum": [
              "ePerA3",
              "fmPerA3",
              "e2PerA3",
              "fm2PerA3"
            ]
          },
          "actualF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "sourcePopulationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "base",
          "phaseId",
          "grid",
          "coreValuesF32Bits",
          "periodicClosureValuesF32Bits",
          "packedSpectrumF64Bits",
          "field",
          "quantityUnit",
          "actualF000F64Bits",
          "sourcePopulationDigest",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeModelFieldInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-model-full-field"
          },
          "model": {
            "$ref": "#/$defs/SG2_NativeDimensionDState"
          },
          "gridSelection": {
            "$ref": "#/$defs/NativeGridNoInput"
          },
          "rawModelModeI32": {
            "enum": [
              0,
              1,
              2,
              3
            ]
          },
          "thermal": {
            "$ref": "#/$defs/NativeThermalSelection"
          },
          "completeElementRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          },
          "anomalousPolicy": {
            "const": "CalcModelDensity-zero-three-column-array"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "model",
          "gridSelection",
          "rawModelModeI32",
          "thermal",
          "completeElementRows",
          "anomalousPolicy",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeModelFieldOutput": {
        "type": "object",
        "properties": {
          "fullField": {
            "$ref": "#/$defs/NativeFullFieldOutput"
          },
          "fullScattering": {
            "$ref": "#/$defs/NativeScatteringOutput"
          },
          "F000PositiveF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "F000NegativeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "F000BeforePattersonSquareF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "allSourceRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          }
        },
        "required": [
          "fullField",
          "fullScattering",
          "F000PositiveF64Bits",
          "F000NegativeF64Bits",
          "F000BeforePattersonSquareF64Bits",
          "allSourceRows"
        ],
        "additionalProperties": false
      },
      "NativeVolumeTransformInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "enum": [
              "vesta3909a-volume-to-structure-factors",
              "vesta3909a-volume-patterson"
            ]
          },
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "grid": {
            "$ref": "#/$defs/NativeFieldGrid"
          },
          "requestedReflections": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "source",
          "grid",
          "requestedReflections",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeVolumeTransformOutput": {
        "type": "object",
        "properties": {
          "fullField": {
            "oneOf": [
              {
                "$ref": "#/$defs/NativeFullFieldOutput"
              },
              {
                "type": "null"
              }
            ]
          },
          "reflections": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "sourceDescriptorDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "transformScaleVolumePerCorePointF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "packedSpectrumF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ComplexF64Bits"
            },
            "minItems": 0
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "fullField",
          "reflections",
          "sourceDescriptorDigest",
          "transformScaleVolumePerCorePointF64Bits",
          "packedSpectrumF64Bits",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativePeakRecord": {
        "type": "object",
        "properties": {
          "sourceCoreIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "fractionalPositionF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "peakValueF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "integralF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "volumeA3F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "metricWeightF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "metricOffsetF64Bits": {
            "$ref": "#/$defs/F64Bits"
          }
        },
        "required": [
          "sourceCoreIndex",
          "fractionalPositionF64Bits",
          "peakValueF64Bits",
          "integralF64Bits",
          "volumeA3F64Bits",
          "metricWeightF64Bits",
          "metricOffsetF64Bits"
        ],
        "additionalProperties": false
      },
      "NativePeakSearchInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-peak-13-half-neighbours"
          },
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "actualC2OColumnsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "requestedCountI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "includeNegative": {
            "type": "boolean"
          },
          "automaticIntegration": {
            "const": "native-Voronoi-t2-method0"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "source",
          "actualC2OColumnsF64Bits",
          "requestedCountI32",
          "includeNegative",
          "automaticIntegration",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeVoronoiInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-voronoi-shell-and-local-refinement"
          },
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "actualC2OColumnsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "orderedPeaks": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePeakRecord"
            },
            "minItems": 1
          },
          "subdivisionsPerAxis": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "metricMethodI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "boundaryToleranceF64Bits": {
            "const": "3eb0c6f7a0b5ed8d"
          },
          "candidateRule": {
            "const": "node-plus-26-wrapped-neighbour-owners"
          },
          "densityRule": {
            "const": "piecewise-constant-original-node-f32"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "source",
          "actualC2OColumnsF64Bits",
          "orderedPeaks",
          "subdivisionsPerAxis",
          "metricMethodI32",
          "boundaryToleranceF64Bits",
          "candidateRule",
          "densityRule",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeNodeAllocation": {
        "type": "object",
        "properties": {
          "coreIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "subsampleOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "tiedOwnerOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 1
          },
          "fractionF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "signedDensityContributionF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "volumeContributionA3F64Bits": {
            "$ref": "#/$defs/F64Bits"
          }
        },
        "required": [
          "coreIndex",
          "subsampleOrdinalZeroBased",
          "tiedOwnerOrdinals",
          "fractionF64Bits",
          "signedDensityContributionF64Bits",
          "volumeContributionA3F64Bits"
        ],
        "additionalProperties": false
      },
      "NativePeakVoronoiOutput": {
        "type": "object",
        "properties": {
          "counterU16": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 65535
            },
            "minItems": 0
          },
          "orderedPeaks": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePeakRecord"
            },
            "minItems": 0
          },
          "initialCoreOwners": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          },
          "initialCoreScoresF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "allAllocations": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeNodeAllocation"
            },
            "minItems": 0
          },
          "boundaryVoxelCount": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "totalSignedIntegralF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "totalAssignedVolumeA3F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "counterU16",
          "orderedPeaks",
          "initialCoreOwners",
          "initialCoreScoresF64Bits",
          "allAllocations",
          "boundaryVoxelCount",
          "totalSignedIntegralF64Bits",
          "totalAssignedVolumeA3F64Bits",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeMeshInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "profile": {
            "const": "vesta3909a-native-isosurface-mesh"
          },
          "orderedActualPhases": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeMeshPhase"
            },
            "minItems": 0
          },
          "actualLayerPopulation": {
            "$ref": "#/$defs/FC_NativeLayerCurrent1"
          },
          "storedLevels": {
            "$ref": "#/$defs/FC_NativeStoredScalarIsoview1"
          },
          "textureSettings": {
            "$ref": "#/$defs/NativeMeshTextureSettings"
          },
          "sourceConsumerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "profile",
          "orderedActualPhases",
          "actualLayerPopulation",
          "storedLevels",
          "textureSettings",
          "sourceConsumerRuleDigest",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeMeshTriangle": {
        "type": "object",
        "properties": {
          "levelOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "sign": {
            "enum": [
              -1,
              1
            ]
          },
          "coreCellIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "nativeCaseId": {
            "type": "integer",
            "minimum": 0,
            "maximum": 255
          },
          "sourceRowTriangleOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4
          },
          "edgeIds": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 11
            },
            "minItems": 3,
            "maxItems": 3
          },
          "vertexIndices": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "levelOrdinal",
          "sign",
          "coreCellIndex",
          "nativeCaseId",
          "sourceRowTriangleOrdinal",
          "edgeIds",
          "vertexIndices"
        ],
        "additionalProperties": false
      },
      "NativeMeshOutput": {
        "type": "object",
        "properties": {
          "verticesCartesianF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 0
          },
          "triangles": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeMeshTriangle"
            },
            "minItems": 0
          },
          "faceNormalsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 0
          },
          "vertexNormalsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 0
          },
          "levelBindings": {
            "$ref": "#/$defs/FC_NativeStoredScalarIsoview1"
          },
          "layerBindings": {
            "$ref": "#/$defs/FC_NativeLayerCurrent1"
          },
          "sourceDescriptorDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "nativeNonfiniteTriangleOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "vertexColorScalarsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 0
          },
          "vertexTextureCoordinatesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 0
          },
          "completeClippedPolygonVertexIndices": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 9007199254740991
              },
              "minItems": 3
            },
            "minItems": 0
          },
          "sourcePhaseByTriangle": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS1_Id"
            },
            "minItems": 0
          },
          "stage": {
            "const": "native-pre-camera-sort-geometry-normals-scalar-texture"
          },
          "levelBranchRanges": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeMeshBranchRange"
            },
            "minItems": 0
          },
          "nativeNonfiniteUsedVertexOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          },
          "nativeNonfiniteUnusedVertexOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          }
        },
        "required": [
          "verticesCartesianF32Bits",
          "triangles",
          "faceNormalsF32Bits",
          "vertexNormalsF32Bits",
          "levelBindings",
          "layerBindings",
          "sourceDescriptorDigest",
          "nativeNonfiniteTriangleOrdinals",
          "parameterDigest",
          "vertexColorScalarsF32Bits",
          "vertexTextureCoordinatesF32Bits",
          "completeClippedPolygonVertexIndices",
          "sourcePhaseByTriangle",
          "stage",
          "levelBranchRanges",
          "nativeNonfiniteUsedVertexOrdinals",
          "nativeNonfiniteUnusedVertexOrdinals"
        ],
        "additionalProperties": false
      },
      "NativeConversionInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-electronic-energy-grid"
          },
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "actualC2OColumnsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "nativeCellSixF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "inputDensityUnit": {
            "enum": [
              "ePerA3",
              "ePerBohr3"
            ]
          },
          "argumentLengthScaleF64Bits": {
            "enum": [
              "3ff0000000000000",
              "3fe0ef050a9737de"
            ]
          },
          "unitProducerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "kineticSelectorI32": {
            "enum": [
              0,
              1
            ]
          },
          "requestedOutputs": {
            "type": "array",
            "items": {
              "enum": [
                "laplacian",
                "kinetic-G",
                "potential-V",
                "total-H",
                "log-abs-density"
              ]
            },
            "minItems": 1,
            "maxItems": 5
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "source",
          "actualC2OColumnsF64Bits",
          "nativeCellSixF32Bits",
          "inputDensityUnit",
          "argumentLengthScaleF64Bits",
          "unitProducerRuleDigest",
          "kineticSelectorI32",
          "requestedOutputs",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeConversionOutput": {
        "type": "object",
        "properties": {
          "stencil27F64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 27,
            "maxItems": 27
          },
          "includedNeighbourOffsets": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS1_Int3"
            },
            "minItems": 20,
            "maxItems": 26
          },
          "laplacianF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "centralDifferenceStencilSurrogateF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "kineticGF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "potentialVF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "totalHF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "logAbsDensityF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "laplacianUnit": {
            "enum": [
              "ePerA5",
              "ePerBohr5"
            ]
          },
          "energyDensityUnit": {
            "const": "hartreePerBohr3"
          },
          "logarithmArgument": {
            "const": "absolute-native-numerical-density"
          },
          "energyScaleF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "nonfiniteDiagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS1_FSNonfiniteDiagnostic"
            },
            "minItems": 0
          },
          "nativeExportSuccessDoesNotCertifyWrite": {
            "const": true
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "stencil27F64Bits",
          "includedNeighbourOffsets",
          "laplacianF64Bits",
          "centralDifferenceStencilSurrogateF64Bits",
          "kineticGF64Bits",
          "potentialVF64Bits",
          "totalHF64Bits",
          "logAbsDensityF64Bits",
          "laplacianUnit",
          "energyDensityUnit",
          "logarithmArgument",
          "energyScaleF64Bits",
          "nonfiniteDiagnostics",
          "nativeExportSuccessDoesNotCertifyWrite",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeDispersionOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeDispersionOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeScatteringOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeScatteringOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeFullFieldOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeFullFieldOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeModelFieldOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeModelFieldOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeVolumeTransformOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeVolumeTransformOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativePeakVoronoiOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativePeakVoronoiOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeMeshOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeMeshOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeConversionOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeConversionOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativePowderBeam": {
        "type": "object",
        "properties": {
          "sourceIndexU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "wavelengthAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "relativeWeightF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "radiation": {
            "enum": [
              "xray-electron-amplitude",
              "neutron-coherent-fm"
            ]
          }
        },
        "required": [
          "sourceIndexU16",
          "wavelengthAF64Bits",
          "relativeWeightF64Bits",
          "radiation"
        ],
        "additionalProperties": false
      },
      "NativePowderShift": {
        "type": "object",
        "properties": {
          "p0TwoThetaDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "p1TwoThetaDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "p2TwoThetaDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "rule": {
            "const": "native-asintheta-with-cos-theta-over2-and-sin-theta"
          }
        },
        "required": [
          "p0TwoThetaDegreeF64Bits",
          "p1TwoThetaDegreeF64Bits",
          "p2TwoThetaDegreeF64Bits",
          "rule"
        ],
        "additionalProperties": false
      },
      "NativePowderFullPhase": {
        "type": "object",
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "model": {
            "$ref": "#/$defs/SG2_NativeDimensionDState"
          },
          "thermal": {
            "$ref": "#/$defs/NativeThermalSelection"
          },
          "elementRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          },
          "dispersionByBeamElement": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/NativeDispersionOutput"
              },
              "minItems": 0
            },
            "minItems": 0
          },
          "actualSymmetryOperatorCount": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "phaseScaleF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "parameters": {
            "$ref": "#/$defs/PowderTch13Parameters"
          },
          "profileKind": {
            "enum": [
              "vesta3909a-TCH",
              "vesta3909a-TCH-Ida"
            ]
          },
          "xShift": {
            "$ref": "#/$defs/NativePowderShift"
          },
          "lorentzParameterM_F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "lorentzParameterP_F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "perObservationFactorsF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "sourcePopulation": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "actualGlobalReferenceMaxIntensityF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "cutRatioDimensionlessF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "cutoffProducerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "enumerationRecipe": {
            "$ref": "#/$defs/NativePowderEnumerationRecipe"
          }
        },
        "required": [
          "phaseId",
          "model",
          "thermal",
          "elementRows",
          "dispersionByBeamElement",
          "actualSymmetryOperatorCount",
          "phaseScaleF64Bits",
          "parameters",
          "profileKind",
          "xShift",
          "lorentzParameterM_F64Bits",
          "lorentzParameterP_F64Bits",
          "perObservationFactorsF64Bits",
          "sourcePopulation",
          "actualGlobalReferenceMaxIntensityF64Bits",
          "cutRatioDimensionlessF64Bits",
          "cutoffProducerRuleDigest",
          "enumerationRecipe"
        ],
        "additionalProperties": false
      },
      "NativePowderFullInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "profile": {
            "const": "vesta3909a-full-native-powder-pipeline"
          },
          "sampleTwoThetaDegreeF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 2
          },
          "beams": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePowderBeam"
            },
            "minItems": 1
          },
          "phases": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePowderFullPhase"
            },
            "minItems": 1
          },
          "normalization": {
            "$ref": "#/$defs/NativePowderNormalization"
          },
          "backgroundF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "profile",
          "sampleTwoThetaDegreeF64Bits",
          "beams",
          "phases",
          "normalization",
          "backgroundF64Bits",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativePowderProfileWitness": {
        "type": "object",
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "reflectionSourceOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "beamSourceIndexU16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "twoThetaDegreeF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "width": {
            "$ref": "#/$defs/NativeTchWidthOutput"
          },
          "equivalentMultiplicity": {
            "type": "integer",
            "minimum": 1,
            "maximum": 2147483647
          },
          "coefficientF64Bits": {
            "$ref": "#/$defs/ComplexF64Bits"
          },
          "startSampleOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "endSampleOrdinalExclusive": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "completeProfileF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 0
          },
          "firstSamplePolicy": {
            "const": "native-index0-remains-initialized-zero"
          },
          "completeProfileStage": {
            "const": "native-weighted-by-observation-beam-phase-once"
          }
        },
        "required": [
          "phaseId",
          "reflectionSourceOrdinalZeroBased",
          "beamSourceIndexU16",
          "twoThetaDegreeF64Bits",
          "width",
          "equivalentMultiplicity",
          "coefficientF64Bits",
          "startSampleOrdinal",
          "endSampleOrdinalExclusive",
          "completeProfileF64Bits",
          "firstSamplePolicy",
          "completeProfileStage"
        ],
        "additionalProperties": false
      },
      "NativePowderFullOutput": {
        "type": "object",
        "properties": {
          "pattern": {
            "$ref": "#/$defs/NativePowderAccumulateOutput"
          },
          "reflectionPopulations": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeReflectionPopulation"
            },
            "minItems": 0
          },
          "allProfileWitnesses": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativePowderProfileWitness"
            },
            "minItems": 0
          },
          "actualElementRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "pattern",
          "reflectionPopulations",
          "allProfileWitnesses",
          "actualElementRows",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeIsoPhaseBuffer": {
        "type": "object",
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "actualStoredVectorF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 0
          },
          "includesActualStoredClosure": {
            "const": true
          },
          "sourceDescriptorDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "phaseId",
          "source",
          "actualStoredVectorF32Bits",
          "includesActualStoredClosure",
          "sourceDescriptorDigest"
        ],
        "additionalProperties": false
      },
      "NativeDefaultIsoInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "profile": {
            "const": "vesta3909a-check-isolevel-global-abs-population"
          },
          "orderedActualPhaseBuffers": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeIsoPhaseBuffer"
            },
            "minItems": 0
          },
          "parameterNF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "preferenceLabel": {
            "const": "Parameter (n): "
          },
          "preferenceProducerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "base",
          "profile",
          "orderedActualPhaseBuffers",
          "parameterNF32Bits",
          "preferenceLabel",
          "preferenceProducerRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeDefaultIsoOutput": {
        "type": "object",
        "properties": {
          "countAllStoredValues": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "signedMinF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "signedMaxF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "absoluteMeanF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "absolutePopulationStdDevF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "storedLevelF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "nativeSignModeI32": {
            "enum": [
              0,
              1,
              2
            ]
          },
          "signMode": {
            "enum": [
              "both",
              "positive",
              "negative"
            ]
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "countAllStoredValues",
          "signedMinF32Bits",
          "signedMaxF32Bits",
          "absoluteMeanF64Bits",
          "absolutePopulationStdDevF64Bits",
          "storedLevelF32Bits",
          "nativeSignModeI32",
          "signMode",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeFractionalCutoffPlane": {
        "type": "object",
        "properties": {
          "normalFractionalF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "distanceFractionalF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "sourcePhaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "sourceSliceId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "producerRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          },
          "orthogonalGeometry": {
            "$ref": "#/$defs/Slice_NativeOrthoCutoffPlaneGeometry"
          }
        },
        "required": [
          "normalFractionalF64Bits",
          "distanceFractionalF64Bits",
          "sourcePhaseId",
          "sourceSliceId",
          "producerRuleDigest",
          "orthogonalGeometry"
        ],
        "additionalProperties": false
      },
      "NativeMeshPhase": {
        "type": "object",
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "colorSource": {
            "oneOf": [
              {
                "$ref": "#/$defs/ScalarSource"
              },
              {
                "type": "null"
              }
            ]
          },
          "nativeVisibility878U8": {
            "type": "integer",
            "minimum": 0,
            "maximum": 255
          },
          "fractionalBoundsInterleavedF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 6,
            "maxItems": 6
          },
          "cutoffPlanes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeFractionalCutoffPlane"
            },
            "minItems": 0
          },
          "effectiveNativeMatrix4": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "effectiveNativeTranslationF32Bits": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "layerQualificationDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "phaseId",
          "source",
          "colorSource",
          "nativeVisibility878U8",
          "fractionalBoundsInterleavedF32Bits",
          "cutoffPlanes",
          "effectiveNativeMatrix4",
          "effectiveNativeTranslationF32Bits",
          "layerQualificationDigest"
        ],
        "additionalProperties": false
      },
      "NativeMeshTextureSettings": {
        "type": "object",
        "properties": {
          "lowerRangeFractionF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "upperRangeFractionF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "signedTextureFactorI8": {
            "type": "integer",
            "minimum": -128,
            "maximum": 127
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "lowerRangeFractionF32Bits",
          "upperRangeFractionF32Bits",
          "signedTextureFactorI8",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativePowderFullOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativePowderFullOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeDefaultIsoOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeDefaultIsoOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "NativeReflectionReaderSettings": {
        "type": "object",
        "properties": {
          "formatCodeI32": {
            "enum": [
              -1,
              3,
              55,
              56,
              57,
              58,
              59
            ]
          },
          "coefficientSelectionI32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 3
          },
          "calculatedF000ElectronF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "calculatedF000NeutronF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "incomingCurrentF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "lineBufferBytes": {
            "const": 150
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "formatCodeI32",
          "coefficientSelectionI32",
          "calculatedF000ElectronF64Bits",
          "calculatedF000NeutronF64Bits",
          "incomingCurrentF000F64Bits",
          "lineBufferBytes",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "NativeReflectionReadInput": {
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FS1_FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/FS1_Id"
          },
          "profile": {
            "const": "vesta3909a-native-reflection-file-reader"
          },
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "settings": {
            "$ref": "#/$defs/NativeReflectionReaderSettings"
          },
          "model": {
            "$ref": "#/$defs/SG2_NativeDimensionDState"
          },
          "completeElementRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ScientificSourceRow"
            },
            "minItems": 0
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "profile",
          "source",
          "settings",
          "model",
          "completeElementRows",
          "maxOutputBytes",
          "maxResidentBytes"
        ],
        "additionalProperties": false
      },
      "NativeSkippedReflectionRow": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/FS1_FSByteSpan"
          },
          "sourceLineOrdinalZeroBased": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "actualScanfAssignmentCountI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "requiredAssignmentCountI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "reason": {
            "enum": [
              "native-scanf-count-skip",
              "native-loop-unrelated-tag",
              "native-missing-column",
              "native-zero-calculated-phase",
              "native-row-filter"
            ]
          }
        },
        "required": [
          "source",
          "sourceLineOrdinalZeroBased",
          "actualScanfAssignmentCountI32",
          "requiredAssignmentCountI32",
          "reason"
        ],
        "additionalProperties": false
      },
      "NativeReflectionReadOutput": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/ReflectionSource"
          },
          "completeRows": {
            "$ref": "#/$defs/NativeReflectionPopulation"
          },
          "actualF000F64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "sourceOrderedSkippedRows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeSkippedReflectionRow"
            },
            "minItems": 0
          },
          "nativeReturnI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "nativeReadDoesNotCertifyAllRowsValid": {
            "const": true
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "source",
          "completeRows",
          "actualF000F64Bits",
          "sourceOrderedSkippedRows",
          "nativeReturnI32",
          "nativeReadDoesNotCertifyAllRowsValid",
          "parameterDigest"
        ],
        "additionalProperties": false
      },
      "NativeReflectionReadOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeReflectionReadOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Sampling_NativePointSampleInput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativePointSampleInput"
      },
      "Sampling_NativePointSampleOutput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativePointSampleOutput"
      },
      "Sampling_NativePointSampleOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Sampling_NativePointSampleOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Sampling_NativeLineProfileInput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativeLineProfileInput"
      },
      "Sampling_NativeLineProfileOutput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativeLineProfileOutput"
      },
      "Sampling_NativeLineProfileOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Sampling_NativeLineProfileOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Sampling_NativeContourLevelsInput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativeContourLevelsInput"
      },
      "Sampling_NativeContourLevelsOutput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativeContourLevelsOutput"
      },
      "Sampling_NativeContourLevelsOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Sampling_NativeContourLevelsOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Sampling_NativeContourSegmentsInput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativeContourSegmentsInput"
      },
      "Sampling_NativeContourSegmentsOutput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativeContourSegmentsOutput"
      },
      "Sampling_NativeContourSegmentsOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Sampling_NativeContourSegmentsOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Sampling_NativePolyIntegralInput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativePolyIntegralInput"
      },
      "Sampling_NativePolyIntegralOutput": {
        "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/NativePolyIntegralOutput"
      },
      "Sampling_NativePolyIntegralOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Sampling_NativePolyIntegralOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Slice_NativeObliqueSliceInput": {
        "$ref": "../slice-geometry/slice-geometry.schema.json#/$defs/NativeObliqueSliceInput"
      },
      "Slice_NativeObliqueSliceOutput": {
        "$ref": "../slice-geometry/slice-geometry.schema.json#/$defs/NativeObliqueSliceOutput"
      },
      "Slice_NativeObliqueSliceOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Slice_NativeObliqueSliceOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Slice_NativePeriodicSliceInput": {
        "$ref": "../slice-geometry/slice-geometry.schema.json#/$defs/NativePeriodicSliceInput"
      },
      "Slice_NativePeriodicSliceOutput": {
        "$ref": "../slice-geometry/slice-geometry.schema.json#/$defs/NativePeriodicSliceOutput"
      },
      "Slice_NativePeriodicSliceOutputOrFailure": {
        "oneOf": [
          {
            "$ref": "#/$defs/Slice_NativePeriodicSliceOutput"
          },
          {
            "$ref": "#/$defs/NativeFailure"
          }
        ]
      },
      "Slice_NativeOrthoCutoffPlaneGeometry": {
        "$ref": "../slice-geometry/slice-geometry.schema.json#/$defs/NativeOrthoCutoffPlaneGeometry"
      },
      "NativeMeshBranchRange": {
        "type": "object",
        "properties": {
          "levelOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "sign": {
            "enum": [
              -1,
              1
            ]
          },
          "firstVertexIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "endVertexIndexExclusive": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "firstTriangleOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "endTriangleOrdinalExclusive": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "levelOrdinal",
          "sign",
          "firstVertexIndex",
          "endVertexIndexExclusive",
          "firstTriangleOrdinal",
          "endTriangleOrdinalExclusive"
        ],
        "additionalProperties": false
      },
      "NativePowderEnumerationRecipe": {
        "type": "object",
        "properties": {
          "sourceTwoThetaRangeDegreeF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "allBeamWavelengthsAF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 1
          },
          "actualDRangeAF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F64Bits"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "actualHklAxisBoundsI32": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            },
            "minItems": 3,
            "maxItems": 3
          },
          "prototypeReflectionDimension": {
            "const": 3
          },
          "rangeRule": {
            "const": "actual-TCH-Calc_d_Range-integer-width-cutoff-extension"
          },
          "populationAuthority": {
            "const": "rederive-from-actual-model-range-beams-profile-compare-all-sourcePopulation-rows"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/FS1_Digest"
          }
        },
        "required": [
          "sourceTwoThetaRangeDegreeF64Bits",
          "allBeamWavelengthsAF64Bits",
          "actualDRangeAF64Bits",
          "actualHklAxisBoundsI32",
          "prototypeReflectionDimension",
          "rangeRule",
          "populationAuthority",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "BoundaryRequest": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "PrepareNativeFourierUi"
              },
              "input": {
                "$ref": "#/$defs/NativeFourierUiInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "CalcNativeGridNo"
              },
              "input": {
                "$ref": "#/$defs/NativeGridNoInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "CalcNativeTchWidth"
              },
              "input": {
                "$ref": "#/$defs/NativeTchWidthInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "AccumulateNativePowderPattern"
              },
              "input": {
                "$ref": "#/$defs/NativePowderAccumulateInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "MaterializeOrderedScalarExpression"
              },
              "input": {
                "$ref": "#/$defs/OrderedExpressionInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "ResampleAffineTrilinear"
              },
              "input": {
                "$ref": "#/$defs/Math_FSResampleAffineInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "LinearCombineFields"
              },
              "input": {
                "$ref": "#/$defs/Math_FSLinearCombineFieldsInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "SampleCartesianLine"
              },
              "input": {
                "$ref": "#/$defs/Math_FSCartesianLineInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "LinearCombineCurves"
              },
              "input": {
                "$ref": "#/$defs/Math_FSLinearCombineCurvesInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "EvaluateNativeDispersion"
              },
              "input": {
                "$ref": "#/$defs/NativeDispersionInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "CalculateNativeStructureFactors"
              },
              "input": {
                "$ref": "#/$defs/NativeScatteringInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "SynthesizeNativeFourierField"
              },
              "input": {
                "$ref": "#/$defs/NativeFourierFieldInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "GenerateNativeModelField"
              },
              "input": {
                "$ref": "#/$defs/NativeModelFieldInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "TransformNativeVolume"
              },
              "input": {
                "$ref": "#/$defs/NativeVolumeTransformInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "SearchNativePeaksAndIntegrate"
              },
              "input": {
                "$ref": "#/$defs/NativePeakSearchInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "IntegrateNativeVoronoi"
              },
              "input": {
                "$ref": "#/$defs/NativeVoronoiInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "BuildNativeIsosurfaceMesh"
              },
              "input": {
                "$ref": "#/$defs/NativeMeshInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "ConvertNativeElectronicEnergy"
              },
              "input": {
                "$ref": "#/$defs/NativeConversionInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "SynthesizeNativePowderPattern"
              },
              "input": {
                "$ref": "#/$defs/NativePowderFullInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "ResolveNativeDefaultIsovalue"
              },
              "input": {
                "$ref": "#/$defs/NativeDefaultIsoInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "ReadNativeReflectionSource"
              },
              "input": {
                "$ref": "#/$defs/NativeReflectionReadInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "SampleNativeFractionalPoint"
              },
              "input": {
                "$ref": "#/$defs/Sampling_NativePointSampleInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "ExportNativeLineProfile"
              },
              "input": {
                "$ref": "#/$defs/Sampling_NativeLineProfileInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "GenerateNativeContourLevels"
              },
              "input": {
                "$ref": "#/$defs/Sampling_NativeContourLevelsInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "BuildNativeContourSegments"
              },
              "input": {
                "$ref": "#/$defs/Sampling_NativeContourSegmentsInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "ExecuteNativePolyIntegralMenu"
              },
              "input": {
                "$ref": "#/$defs/Sampling_NativePolyIntegralInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "BuildNativeObliqueSlice"
              },
              "input": {
                "$ref": "#/$defs/Slice_NativeObliqueSliceInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schemaVersion": {
                "const": "FS/2"
              },
              "operation": {
                "const": "BuildNativePeriodicSlice"
              },
              "input": {
                "$ref": "#/$defs/Slice_NativePeriodicSliceInput"
              }
            },
            "required": [
              "schemaVersion",
              "operation",
              "input"
            ],
            "additionalProperties": false
          }
        ]
      }
    },
    "x-imports": [
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json",
        "relativePath": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json",
        "sha256": "adf539aac53ddd41b46651886b00e9d5394a96c36b83f74aeb6e35984898000a"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-fractional-grid-current.schema.json",
        "relativePath": "../../../../../20261115-092025-p1-03-completion/native-fractional-grid-current.schema.json",
        "sha256": "024353fc084874aa0bf0b7aa41225129672ec9fd8cff0ed1fcdd2b746e408ae9"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-scalar-grid-current.schema.json",
        "relativePath": "../../../../../20261115-092025-p1-03-completion/native-scalar-grid-current.schema.json",
        "sha256": "aae3f7260198c218b376c35b756e102d9acedf7d66a745b0df526202651c69af"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-stored-isoview.schema.json",
        "relativePath": "../../../../../20261115-092025-p1-03-completion/native-stored-isoview.schema.json",
        "sha256": "5cdc2d8ddd4c1f146c984b61bc2a4f869d96d5411f870e788aa61cd4dc6387db"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/functional-additions.schema.json",
        "relativePath": "functional-additions.schema.json",
        "sha256": "89a82dfcab19f813e65c1c89c7a308d1e37a02239ed6e038fd3706af7a002a76"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json",
        "relativePath": "../sg/sg2.schema.json",
        "sha256": "5d24de755b9e937a6d0e18f02ad967889e8a8cf7221cad8c41b326c97647f688"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-layer-current.schema.json",
        "relativePath": "../../../../../20261115-092025-p1-03-completion/native-layer-current.schema.json",
        "sha256": "31e8aa75b924339c9168994f7a8615b4540a52c3294c07a694613a66874a42de"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/science-supplement/native-sampling.schema.json",
        "relativePath": "../../science-supplement/native-sampling.schema.json",
        "sha256": "5751d8dc422c31e9e215c0488decf69cd5fd250252b67c0b9fa6bc72a9e927f6"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/slice-geometry/slice-geometry.schema.json",
        "relativePath": "../slice-geometry/slice-geometry.schema.json",
        "sha256": "7a6caa11871d38ae83919c6f729155742fe0dae79d18852d8111fb7102caba92"
      }
    ],
    "x-semanticRules": [
      "Native source identities preserve actual known licenseTextDigest (VESTA LICENSE c6690795ed2edf9f2c45d6a075d1964e242d5ef7ba1333891ece1da73b2bf1c1 where applicable) separately from writtenPermissionEvidenceDigest. The former is text identity, not authorization. Licensing is deferred; own mathematical/caller input may explicitlynot-applicable. Scientific source/producer/consumer/unit qualification remains required; no syntheticlicense digest or permission grant.",
      "Independent successor namespace. FS/1, bounded3 and the177 operation signatures are unchanged; these are future design constructors, not implemented protocol3 methods.",
      "Every original source format is a named typed variant plus exact actual source bytes/materialized rows. Import grammar qualification remains per declared dialect; no options JSON or absent phase guessed tozero.",
      "Fourier firstSource.readerSelection equalsuiSelection; secondSource exists only forUI3 and hasreaderSelection3. Eachsource.base/phaseId mustequalrootbase/phaseId andactualcurrenttypedsource owner, withall references rederivedfromoneimmutable current snapshot; sourceDigest alone cannot grantcurrentauthority. Onephase/context/amplitudeunit acrosssources. ActualF000 is independent currentUI scalar, not recomputed difference. Raw backendmode is always0.",
      "NativeGridNo population must equal complete actual qualified selected Wyckoff/current symmetry populations, independently rederived fromsource/current state. Digest equality supplied bycaller cannot grant authority. All matrix/axis/resolution bits decodefinite, positive admitted counts, checked integer arithmetic and budgets.",
      "Powder13 name/offset/count/unit record is exact. GetParam returns10, names13. rs/rd are native dimensionless profile coefficients with author geometry reference; preserve raw values and actual algorithm, no silent degree/radian/FCJ substitution. Width consumesU..Ye and rawaxis slot; rs/rd are not consumed bythiswidth body.",
      "Native powder accumulation usesallphases inactualorder, perreflection absF2*multiplicity*beamWeight*phaseScale*perObservationFactor*materializedProfile; actualReflection88 correction affectsrawCalcIcmax/cutoff, not finalpointmultiplication. materializedProfileStage explicitlymeansbareunweightedphi; completeNativeProfileWitnessalreadyhasobservation*beam*phaseonceandisnotmultipliedagain. All sampledarrays exactlymatchsamplecount. Sharedscale=actualtarget/max(0,rawaggregate) withtarget100 for noobservedpattern oractualstoredobservedMax otherwise; addbackground onlyaftersharedscale. Expose everyraw/scaledphase, aggregate/background/total. Zero max produces sourcefailure; valid finiteproductoutput failsatomically ratherthaninventingzero. This isrelativeDisplayIntensity, not absolute radiationintensity or externalRIETAN1e5.",
      "Ordered native expression uses source entry order, originalformat selector and explicit f32 arithmetic. Firstentry initialization ignores multiply/divide prefix per source; add usesfma32; multiply anddivide retain separate operations. Result physical quantity/unit needs qualified derivation; native unqualified scalar remainsuninterpreted.",
      "Physical FSFieldBinding and original current numerical SourceEntity types remain distinct; no retagging native numerical grids into physical Fields. All descriptor/resource/owner/context/reference roles must validate under complete sourcevisitor.",
      "Native expression profiles cannot select corrected indexed selector; correctedprofile must identify correction. Sourceformat and selector taggedvariant must match. 3.6 source absent is not published as a qualified3.0 variant.",
      "Finite/count/domain/closure/geometric/quantity/source compatibility and all array lengths are semantic checks beyond JSONSchema. Failed/canceledjobs preserveinput and release owncandidate resources atomically.",
      "Native fullD scattering consumes completeactualSG2state including I16 orbit and U16 siteflags; source selection isactual217recordalgorithm, not an upgradedall-isotope database. EveryElementrow isrederivedfrombyteboundsource, notcallerassertedselfauthority. AllsourceD/hkl/coordinate lengthsagree. Nativefield success requiresactualD3: VolDatac/10/14 arethree divisions;18isvectorpointer; FFVTreadsD i32s, soD4..6 shape conflictsreturnE_NATIVE_GRID_DIMENSION_LAYOUTbeforeunsafeaccess. This isstaticlayoutfailuredesign, not observedoriginalcrash; fullD scatteringstillavailable.",
      "Fullfieldgenerators exposeallcoefficients, fullcoreandclosure; source-onlyunqualifiednativevaluesneverbecomephysicalFSFieldbyretagging. VolumePattersonconsumesactualgrid, scaleV/N beforeforwardFFTand1/V afterbackward; noabsolute-inputcorrection. Peak/Voronoi exposeallcounters/owners/scores/allocations; onlyboundaryt3subsamplesplitsofpiecewiseconstantnodevalueandnativecomponentwiseminimage, notglobaltriclinicCVP.",
      "Electronicenergy requestedoutputs mapexact5nativeflags:laplacian,G,V,H,logabsrho; sourceinpute/A3ore/bohr3determinesnative1ora0lengthscale, laplacianmatchinglength^-5andenergieshartree/bohr3. Nativefailure/nonfiniteoutputmustretainfullclassification; nozero-for-infinitysubstitution.",
      "All complete output/oracle arrays are compared, with one exact path/SHA binding and perfixture comparison. Source-derived controls are not independent runtime holdouts. Future execution/performance qualification does not make these public payload types unspecified."
    ],
    "x-designOnly": true,
    "x-productImplemented": false,
    "x-nativeRuntimeValidated": false
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/render-consumption/render-packet.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:draft:render:native-isosurface:1",
    "$defs": {
      "FS_NativeMeshOutput": {
        "type": "object",
        "properties": {
          "verticesCartesianF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_Vec3F32Bits"
            },
            "minItems": 0
          },
          "triangles": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_NativeMeshTriangle"
            },
            "minItems": 0
          },
          "faceNormalsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_Vec3F32Bits"
            },
            "minItems": 0
          },
          "vertexNormalsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_Vec3F32Bits"
            },
            "minItems": 0
          },
          "levelBindings": {
            "$ref": "#/$defs/FS_FC_NativeStoredScalarIsoview1"
          },
          "layerBindings": {
            "$ref": "#/$defs/FS_FC_NativeLayerCurrent1"
          },
          "sourceDescriptorDigest": {
            "$ref": "#/$defs/FS_FS1_Digest"
          },
          "nativeNonfiniteTriangleOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          },
          "parameterDigest": {
            "$ref": "#/$defs/FS_FS1_Digest"
          },
          "vertexColorScalarsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_F32Bits"
            },
            "minItems": 0
          },
          "vertexTextureCoordinatesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_F32Bits"
            },
            "minItems": 0
          },
          "completeClippedPolygonVertexIndices": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 9007199254740991
              },
              "minItems": 3
            },
            "minItems": 0
          },
          "sourcePhaseByTriangle": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_FS1_Id"
            },
            "minItems": 0
          },
          "stage": {
            "const": "native-pre-camera-sort-geometry-normals-scalar-texture"
          },
          "levelBranchRanges": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FS_NativeMeshBranchRange"
            },
            "minItems": 0
          },
          "nativeNonfiniteUsedVertexOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          },
          "nativeNonfiniteUnusedVertexOrdinals": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 0
          }
        },
        "required": [
          "verticesCartesianF32Bits",
          "triangles",
          "faceNormalsF32Bits",
          "vertexNormalsF32Bits",
          "levelBindings",
          "layerBindings",
          "sourceDescriptorDigest",
          "nativeNonfiniteTriangleOrdinals",
          "parameterDigest",
          "vertexColorScalarsF32Bits",
          "vertexTextureCoordinatesF32Bits",
          "completeClippedPolygonVertexIndices",
          "sourcePhaseByTriangle",
          "stage",
          "levelBranchRanges",
          "nativeNonfiniteUsedVertexOrdinals",
          "nativeNonfiniteUnusedVertexOrdinals"
        ],
        "additionalProperties": false
      },
      "FS_NativeMeshBranchRange": {
        "type": "object",
        "properties": {
          "levelOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "sign": {
            "enum": [
              -1,
              1
            ]
          },
          "firstVertexIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "endVertexIndexExclusive": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "firstTriangleOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "endTriangleOrdinalExclusive": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "levelOrdinal",
          "sign",
          "firstVertexIndex",
          "endVertexIndexExclusive",
          "firstTriangleOrdinal",
          "endTriangleOrdinalExclusive"
        ],
        "additionalProperties": false
      },
      "FS_FS1_Id": {
        "$ref": "#/$defs/IM2_1_Id"
      },
      "IM2_1_Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "title": "Id"
      },
      "FS_F32Bits": {
        "type": "string",
        "pattern": "^[0-9a-f]{8}$"
      },
      "FS_FS1_Digest": {
        "$ref": "#/$defs/IM2_1_Digest"
      },
      "IM2_1_Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$",
        "title": "Digest"
      },
      "FS_FC_NativeLayerCurrent1": {
        "$ref": "#/$defs/IM7_2_ROOT"
      },
      "IM7_2_ROOT": {
        "type": "object",
        "properties": {
          "schemaVersion": {
            "const": "FC-native-layer-current-1"
          },
          "captureRole": {
            "const": "actual-current-native-3D-layer-population"
          },
          "document": {
            "type": "object",
            "properties": {
              "kind": {
                "const": "EntityRef"
              },
              "objectKind": {
                "const": "document"
              },
              "objectId": {
                "type": "string",
                "minLength": 1
              }
            },
            "required": [
              "kind",
              "objectKind",
              "objectId"
            ],
            "additionalProperties": false
          },
          "orderedEntries": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "owner": {
                  "oneOf": [
                    {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "const": "EntityRef"
                        },
                        "objectKind": {
                          "const": "phase"
                        },
                        "objectId": {
                          "type": "string",
                          "minLength": 1
                        }
                      },
                      "required": [
                        "kind",
                        "objectKind",
                        "objectId"
                      ],
                      "additionalProperties": false
                    },
                    {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "const": "EntityRef"
                        },
                        "objectKind": {
                          "const": "frame"
                        },
                        "objectId": {
                          "type": "string",
                          "minLength": 1
                        }
                      },
                      "required": [
                        "kind",
                        "objectKind",
                        "objectId"
                      ],
                      "additionalProperties": false
                    },
                    {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "const": "EntityRef"
                        },
                        "objectKind": {
                          "const": "source-entity"
                        },
                        "objectId": {
                          "type": "string",
                          "minLength": 1
                        }
                      },
                      "required": [
                        "kind",
                        "objectKind",
                        "objectId"
                      ],
                      "additionalProperties": false
                    }
                  ]
                },
                "position": {
                  "type": "object",
                  "properties": {
                    "selector": {
                      "type": "object",
                      "properties": {
                        "capturedSelector": {
                          "type": "integer",
                          "minimum": -2147483648,
                          "maximum": 2147483647
                        },
                        "reference": {
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "const": "internal-cartesian"
                                }
                              },
                              "required": [
                                "kind"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "const": "entry"
                                },
                                "target": {
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "const": "EntityRef"
                                        },
                                        "objectKind": {
                                          "const": "phase"
                                        },
                                        "objectId": {
                                          "type": "string",
                                          "minLength": 1
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "objectKind",
                                        "objectId"
                                      ],
                                      "additionalProperties": false
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "const": "EntityRef"
                                        },
                                        "objectKind": {
                                          "const": "frame"
                                        },
                                        "objectId": {
                                          "type": "string",
                                          "minLength": 1
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "objectKind",
                                        "objectId"
                                      ],
                                      "additionalProperties": false
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "const": "EntityRef"
                                        },
                                        "objectKind": {
                                          "const": "source-entity"
                                        },
                                        "objectId": {
                                          "type": "string",
                                          "minLength": 1
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "objectKind",
                                        "objectId"
                                      ],
                                      "additionalProperties": false
                                    }
                                  ]
                                }
                              },
                              "required": [
                                "kind",
                                "target"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "const": "unresolved"
                                },
                                "reason": {
                                  "enum": [
                                    "out-of-population",
                                    "ambiguous-native-instance",
                                    "unqualified-source-index"
                                  ]
                                }
                              },
                              "required": [
                                "kind",
                                "reason"
                              ],
                              "additionalProperties": false
                            }
                          ]
                        }
                      },
                      "required": [
                        "capturedSelector",
                        "reference"
                      ],
                      "additionalProperties": false
                    },
                    "currentAnchor": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "const": "F64Bits"
                          },
                          "ieeeHexBE": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{16}$"
                          }
                        },
                        "required": [
                          "kind",
                          "ieeeHexBE"
                        ],
                        "additionalProperties": false
                      },
                      "minItems": 3,
                      "maxItems": 3
                    },
                    "referenceAnchor": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "const": "F64Bits"
                          },
                          "ieeeHexBE": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{16}$"
                          }
                        },
                        "required": [
                          "kind",
                          "ieeeHexBE"
                        ],
                        "additionalProperties": false
                      },
                      "minItems": 3,
                      "maxItems": 3
                    }
                  },
                  "required": [
                    "selector",
                    "currentAnchor",
                    "referenceAnchor"
                  ],
                  "additionalProperties": false
                },
                "orientation": {
                  "type": "object",
                  "properties": {
                    "selector": {
                      "type": "object",
                      "properties": {
                        "capturedSelector": {
                          "type": "integer",
                          "minimum": -2147483648,
                          "maximum": 2147483647
                        },
                        "reference": {
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "const": "internal-cartesian"
                                }
                              },
                              "required": [
                                "kind"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "const": "entry"
                                },
                                "target": {
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "const": "EntityRef"
                                        },
                                        "objectKind": {
                                          "const": "phase"
                                        },
                                        "objectId": {
                                          "type": "string",
                                          "minLength": 1
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "objectKind",
                                        "objectId"
                                      ],
                                      "additionalProperties": false
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "const": "EntityRef"
                                        },
                                        "objectKind": {
                                          "const": "frame"
                                        },
                                        "objectId": {
                                          "type": "string",
                                          "minLength": 1
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "objectKind",
                                        "objectId"
                                      ],
                                      "additionalProperties": false
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
                                          "const": "EntityRef"
                                        },
                                        "objectKind": {
                                          "const": "source-entity"
                                        },
                                        "objectId": {
                                          "type": "string",
                                          "minLength": 1
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "objectKind",
                                        "objectId"
                                      ],
                                      "additionalProperties": false
                                    }
                                  ]
                                }
                              },
                              "required": [
                                "kind",
                                "target"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "const": "unresolved"
                                },
                                "reason": {
                                  "enum": [
                                    "out-of-population",
                                    "ambiguous-native-instance",
                                    "unqualified-source-index"
                                  ]
                                }
                              },
                              "required": [
                                "kind",
                                "reason"
                              ],
                              "additionalProperties": false
                            }
                          ]
                        }
                      },
                      "required": [
                        "capturedSelector",
                        "reference"
                      ],
                      "additionalProperties": false
                    },
                    "storedFlagBits": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{8}$"
                    },
                    "currentFirst": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "const": "F64Bits"
                          },
                          "ieeeHexBE": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{16}$"
                          }
                        },
                        "required": [
                          "kind",
                          "ieeeHexBE"
                        ],
                        "additionalProperties": false
                      },
                      "minItems": 3,
                      "maxItems": 3
                    },
                    "referenceFirst": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "const": "F64Bits"
                          },
                          "ieeeHexBE": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{16}$"
                          }
                        },
                        "required": [
                          "kind",
                          "ieeeHexBE"
                        ],
                        "additionalProperties": false
                      },
                      "minItems": 3,
                      "maxItems": 3
                    },
                    "currentSecond": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "const": "F64Bits"
                          },
                          "ieeeHexBE": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{16}$"
                          }
                        },
                        "required": [
                          "kind",
                          "ieeeHexBE"
                        ],
                        "additionalProperties": false
                      },
                      "minItems": 3,
                      "maxItems": 3
                    },
                    "referenceSecond": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "const": "F64Bits"
                          },
                          "ieeeHexBE": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{16}$"
                          }
                        },
                        "required": [
                          "kind",
                          "ieeeHexBE"
                        ],
                        "additionalProperties": false
                      },
                      "minItems": 3,
                      "maxItems": 3
                    }
                  },
                  "required": [
                    "selector",
                    "storedFlagBits",
                    "currentFirst",
                    "referenceFirst",
                    "currentSecond",
                    "referenceSecond"
                  ],
                  "additionalProperties": false
                },
                "effectiveNativeMatrix4": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 16,
                  "maxItems": 16
                },
                "effectiveNativeTranslation": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F32Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{8}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                }
              },
              "required": [
                "owner",
                "position",
                "orientation",
                "effectiveNativeMatrix4",
                "effectiveNativeTranslation"
              ],
              "additionalProperties": false
            }
          }
        },
        "required": [
          "schemaVersion",
          "captureRole",
          "document",
          "orderedEntries"
        ],
        "additionalProperties": false,
        "title": "Complete actual current selected3D native layer population; immutable source observations separate"
      },
      "FS_FC_NativeStoredScalarIsoview1": {
        "$ref": "#/$defs/IM9_3_ROOT"
      },
      "IM9_3_ROOT": {
        "type": "object",
        "properties": {
          "schemaVersion": {
            "const": "FC-native-stored-isoview-1"
          },
          "target": {
            "type": "object",
            "properties": {
              "kind": {
                "const": "EntityRef"
              },
              "objectKind": {
                "const": "source-entity"
              },
              "objectId": {
                "type": "string",
                "minLength": 1
              }
            },
            "required": [
              "kind",
              "objectKind",
              "objectId"
            ],
            "additionalProperties": false
          },
          "levels": {
            "type": "array",
            "minItems": 1,
            "items": {
              "type": "object",
              "properties": {
                "storedValueF32Bits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{8}$"
                },
                "signMode": {
                  "enum": [
                    "positive",
                    "negative",
                    "both"
                  ]
                }
              },
              "required": [
                "storedValueF32Bits",
                "signMode"
              ],
              "additionalProperties": false
            }
          },
          "levelMeaning": {
            "const": "stored-scalar-only"
          },
          "ruleId": {
            "const": "draft.native-stored-scalar-isoview"
          },
          "ruleVersion": {
            "const": "1"
          }
        },
        "required": [
          "schemaVersion",
          "target",
          "levels",
          "levelMeaning",
          "ruleId",
          "ruleVersion"
        ],
        "additionalProperties": false
      },
      "FS_Vec3F32Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/FS_F32Bits"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "FS_NativeMeshTriangle": {
        "type": "object",
        "properties": {
          "levelOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "sign": {
            "enum": [
              -1,
              1
            ]
          },
          "coreCellIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "nativeCaseId": {
            "type": "integer",
            "minimum": 0,
            "maximum": 255
          },
          "sourceRowTriangleOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4
          },
          "edgeIds": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 11
            },
            "minItems": 3,
            "maxItems": 3
          },
          "vertexIndices": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "levelOrdinal",
          "sign",
          "coreCellIndex",
          "nativeCaseId",
          "sourceRowTriangleOrdinal",
          "edgeIds",
          "vertexIndices"
        ],
        "additionalProperties": false
      },
      "SG_NativeCamera": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "sceneMatrixLogicalRows": {
            "$ref": "#/$defs/SG_Matrix4Rows"
          },
          "shiftViewX": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "view-coordinate",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "shiftViewY": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "view-coordinate",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "thirdSceneStoredF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "source-unassigned-preserved",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "zoomF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "projectionSelectorBit21": {
            "enum": [
              0,
              1
            ]
          },
          "projectionZScaleF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "localTranslationA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": -3.4028234663852886e+38,
              "maximum": 3.4028234663852886e+38,
              "x-unit": "angstrom",
              "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "normalizationF64": {
            "type": "number",
            "minimum": -1.7976931348623157e+308,
            "maximum": 1.7976931348623157e+308,
            "x-unit": "1/angstrom"
          },
          "viewportLogicalWidth": {
            "type": "integer",
            "minimum": 1
          },
          "viewportLogicalHeight": {
            "type": "integer",
            "minimum": 1
          },
          "deviceScaleF64": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "aspectF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "aspectScaleF32": {
            "type": "number",
            "minimum": -3.4028234663852886e+38,
            "maximum": 3.4028234663852886e+38,
            "x-unit": "dimensionless",
            "x-invariant": "exact finite binary32 value promoted to JSONnumber; rawWord must match wherever supplied"
          },
          "projectionContext": {
            "enum": [
              "capture-restored-normal",
              "tiled-capture",
              "explicit-canonical-camera"
            ]
          },
          "nearView": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          },
          "farView": {
            "type": "number",
            "exclusiveMinimum": 0,
            "maximum": 1.7976931348623157e+308
          }
        },
        "required": [
          "sceneMatrixLogicalRows",
          "shiftViewX",
          "shiftViewY",
          "thirdSceneStoredF32",
          "zoomF32",
          "projectionSelectorBit21",
          "projectionZScaleF32",
          "localTranslationA",
          "normalizationF64",
          "viewportLogicalWidth",
          "viewportLogicalHeight",
          "deviceScaleF64",
          "aspectF32",
          "aspectScaleF32",
          "projectionContext",
          "nearView",
          "farView"
        ],
        "x-invariants": [
          "source Mview=T(0,0,-5)*T(shiftX,shiftY,0)*Sxy,z*[Sz(PROJTscalar)ifbit21]*SceneM*Tlocal",
          "sxy=f32(f64(zoom)*normalization*f64(aspectScale));sz=normalization*f64(f32(min(zoom,4)*aspectScale))",
          "aspect=f32(width/height);aspectScale=1 ifwidth<=height elsef32(height/width); physicalviewporttrunc(logical*deviceScale)",
          "full16f64affine source preserved;not forcedintoquaternion;shiftView notpanWorldA",
          "PROJTscalar ismodelviewzscale,notFOV;near<far;context-specificsourceprojection separatelyqualified",
          "capturesourcebit0 ortho±1,y±1/aspect;bit1 frustum±.2,y±.2/aspect,near1far10 atrestore;tile contexts source-specific",
          "bounds-derivedlocaltranslation/normalizationinputs tiedsamegeometryrevision,not stale sourcecamera history"
        ]
      },
      "SG_Matrix4Rows": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/SG_Vec4"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "SG_Vec4": {
        "type": "array",
        "items": {
          "type": "number",
          "minimum": -1.7976931348623157e+308,
          "maximum": 1.7976931348623157e+308,
          "x-unit": "dimensionless"
        },
        "minItems": 4,
        "maxItems": 4
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "F32Bits": {
        "type": "string",
        "pattern": "^[0-9a-f]{8}$"
      },
      "F64Bits": {
        "type": "string",
        "pattern": "^[0-9a-f]{16}$"
      },
      "Vec3F32Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/F32Bits"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Vec3F64Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/F64Bits"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "RGBAU8": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 0,
          "maximum": 255
        },
        "minItems": 4,
        "maxItems": 4
      },
      "RGBU8": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 0,
          "maximum": 255
        },
        "minItems": 3,
        "maxItems": 3
      },
      "Triangle": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 0,
          "maximum": 9007199254740991
        },
        "minItems": 3,
        "maxItems": 3
      },
      "NativeLevelAppearance": {
        "type": "object",
        "properties": {
          "levelOrdinal": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "rgbSourceU8": {
            "$ref": "#/$defs/RGBU8"
          },
          "alphaFacingU8": {
            "type": "integer",
            "minimum": 0,
            "maximum": 255
          },
          "alphaGrazingU8": {
            "type": "integer",
            "minimum": 0,
            "maximum": 255
          }
        },
        "required": [
          "levelOrdinal",
          "rgbSourceU8",
          "alphaFacingU8",
          "alphaGrazingU8"
        ],
        "additionalProperties": false
      },
      "NativeSurfaceDisplay": {
        "type": "object",
        "properties": {
          "scene130U8": {
            "type": "integer",
            "minimum": 0,
            "maximum": 255
          },
          "scene5B0U32": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "scene354U16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "scene356U16": {
            "type": "integer",
            "minimum": 0,
            "maximum": 65535
          },
          "scene2F2SelectorI8": {
            "type": "integer",
            "minimum": -128,
            "maximum": 127
          },
          "surfaceSpecularRGBAU8": {
            "$ref": "#/$defs/RGBAU8"
          },
          "surfaceShininessF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "exportSpecularRGB3DCU8": {
            "$ref": "#/$defs/RGBU8"
          },
          "exportShininess3E0F32Bits": {
            "$ref": "#/$defs/F32Bits"
          }
        },
        "required": [
          "scene130U8",
          "scene5B0U32",
          "scene354U16",
          "scene356U16",
          "scene2F2SelectorI8",
          "surfaceSpecularRGBAU8",
          "surfaceShininessF32Bits",
          "exportSpecularRGB3DCU8",
          "exportShininess3E0F32Bits"
        ],
        "additionalProperties": false
      },
      "OwnedPalette": {
        "type": "object",
        "properties": {
          "profile": {
            "enum": [
              "native-qualified-actual-binding",
              "corrected-owned-source-LUT"
            ]
          },
          "kind": {
            "enum": [
              "builtin-source-LUT",
              "explicit-custom-RGB1024",
              "explicit-constant-RGB"
            ]
          },
          "sourceSlot": {
            "oneOf": [
              {
                "type": "integer",
                "minimum": 0,
                "maximum": 6
              },
              {
                "type": "null"
              }
            ]
          },
          "format": {
            "enum": [
              "RGB-u8",
              "LUMINANCE-u8"
            ]
          },
          "width": {
            "enum": [
              1,
              256,
              1024
            ]
          },
          "bytesHex": {
            "type": "string",
            "pattern": "^[0-9a-f]+$"
          },
          "contentDigest": {
            "$ref": "#/$defs/Digest"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          },
          "samplerWrapS": {
            "enum": [
              "repeat",
              "clamp-to-edge",
              "clamp"
            ]
          },
          "minFilter": {
            "const": "nearest"
          },
          "magFilter": {
            "const": "nearest"
          },
          "nativeBindingQualificationDigest": {
            "oneOf": [
              {
                "$ref": "#/$defs/Digest"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "profile",
          "kind",
          "sourceSlot",
          "format",
          "width",
          "bytesHex",
          "contentDigest",
          "sourceRuleDigest",
          "samplerWrapS",
          "minFilter",
          "magFilter",
          "nativeBindingQualificationDigest"
        ],
        "additionalProperties": false
      },
      "NativeBoundaryDisplayMesh": {
        "type": "object",
        "properties": {
          "meshId": {
            "type": "string",
            "minLength": 1
          },
          "phaseId": {
            "type": "string",
            "minLength": 1
          },
          "sliceId": {
            "type": "string",
            "minLength": 1
          },
          "kind": {
            "enum": [
              "bbox-face",
              "cutoff-plane",
              "section"
            ]
          },
          "sourceNormalF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "verticesCartesianF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            }
          },
          "triangles": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Triangle"
            }
          },
          "textureCoordinatesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            }
          },
          "rgbaSourceU8": {
            "$ref": "#/$defs/RGBAU8"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "meshId",
          "phaseId",
          "sliceId",
          "kind",
          "sourceNormalF64Bits",
          "verticesCartesianF32Bits",
          "triangles",
          "textureCoordinatesF32Bits",
          "rgbaSourceU8",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "DepthState": {
        "type": "object",
        "properties": {
          "testEnabled": {
            "type": "boolean"
          },
          "writeEnabled": {
            "type": "boolean"
          },
          "comparison": {
            "enum": [
              "never",
              "less",
              "equal",
              "less-equal",
              "greater",
              "not-equal",
              "greater-equal",
              "always"
            ]
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "testEnabled",
          "writeEnabled",
          "comparison",
          "sourceRuleDigest"
        ],
        "additionalProperties": false
      },
      "RenderNativeIsosurfacePacketRequest": {
        "type": "object",
        "properties": {
          "namespace": {
            "const": "vestacraft.render.native-isosurface/1"
          },
          "profile": {
            "const": "vesta3909a-source-final-surface-consumption"
          },
          "preCameraMesh": {
            "$ref": "#/$defs/FS_NativeMeshOutput"
          },
          "camera": {
            "$ref": "#/$defs/SG_NativeCamera"
          },
          "depthState": {
            "$ref": "#/$defs/DepthState"
          },
          "inheritedPolygonMode": {
            "enum": [
              "fill",
              "line",
              "point"
            ]
          },
          "levelAppearanceInSourceOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeLevelAppearance"
            }
          },
          "display": {
            "$ref": "#/$defs/NativeSurfaceDisplay"
          },
          "palette": {
            "oneOf": [
              {
                "$ref": "#/$defs/OwnedPalette"
              },
              {
                "type": "null"
              }
            ]
          },
          "boundaryDisplayMeshes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeBoundaryDisplayMesh"
            }
          },
          "ownerId": {
            "type": "string",
            "minLength": 1
          },
          "revision": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          },
          "maximumPacketBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "namespace",
          "profile",
          "preCameraMesh",
          "camera",
          "depthState",
          "inheritedPolygonMode",
          "levelAppearanceInSourceOrder",
          "display",
          "palette",
          "boundaryDisplayMeshes",
          "ownerId",
          "revision",
          "sourceRuleDigest",
          "maximumPacketBytes"
        ],
        "additionalProperties": false,
        "x-invariants": [
          "FS-pre-camera meshstage/rule/owner/revisionmatchesexactactualpopulation;SourceCurrenthistorynotreplayed",
          "levelappearancecount==storedlevels.count,levelOrdinal==sourceorder;levelBranchRangescaptureactualcompletevertexranges,neverinferunusedvertexownershipfromtriangles",
          "palettebytes length=width*(formatRGB?3:1),contentDigest=SHA256(actualbytes);builtinmatchespinnedsourceLUTordinal;nativeprofilealsoqualifiedactualbindingdigest",
          "boundarymeshtriangleindicesinrange,texcoordscount==verticescount;filledmeshfromactualCreate_Slicesproducer,notartificialfanofclippedisosurfaceedges",
          "usedvertexnonfinitegeometry/normals/colorarithmetic ->failure. Unusednonfinite words remainpremesh;nullablederivedRGBAonlyforunusedvertex withUNUSED_VERTEX_NONFINITEdiagnostic;renderpacketdoesnotpretendallcachedRGBAbytesareknown",
          "sourceScene3e4/3e8usedGLsurface,3dc/3e0keptdistinctVRMLprofile;noUIlabelmaterialcollapse",
          "maximumpacketbytes>0;doesnotmutatedocument/render/nativeGLstate,plansonly"
        ]
      },
      "BoundaryPass": {
        "type": "object",
        "properties": {
          "passValue": {
            "enum": [
              -1,
              0,
              1
            ]
          },
          "position": {
            "enum": [
              "before-isosurface",
              "after-isosurface"
            ]
          },
          "admittedMeshIds": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "rejectedMeshIds": {
            "type": "array",
            "items": {
              "type": "string",
              "minLength": 1
            }
          },
          "normalZeroInBothPasses": {
            "const": true
          },
          "lighting": {
            "const": "disabled"
          },
          "textureEnvironment": {
            "const": "modulate"
          }
        },
        "required": [
          "passValue",
          "position",
          "admittedMeshIds",
          "rejectedMeshIds",
          "normalZeroInBothPasses",
          "lighting",
          "textureEnvironment"
        ],
        "additionalProperties": false
      },
      "SurfaceMaterialPlan": {
        "type": "object",
        "properties": {
          "specularNormalizedRGBAF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 4,
            "maxItems": 4
          },
          "shininessF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "faces": {
            "const": "front-and-back"
          },
          "polygonMode": {
            "enum": [
              "fill",
              "line",
              "point"
            ]
          },
          "lineWidthF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "pointSizeF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "blendSrc": {
            "const": "source-alpha"
          },
          "blendDst": {
            "const": "one-minus-source-alpha"
          },
          "depthState": {
            "$ref": "#/$defs/DepthState"
          }
        },
        "required": [
          "specularNormalizedRGBAF32Bits",
          "shininessF32Bits",
          "faces",
          "polygonMode",
          "lineWidthF32Bits",
          "pointSizeF32Bits",
          "blendSrc",
          "blendDst",
          "depthState"
        ],
        "additionalProperties": false
      },
      "PacketDiagnostic": {
        "type": "object",
        "properties": {
          "code": {
            "enum": [
              "E_REFERENCE_FAILURE",
              "E_SOURCE_REFERENCE",
              "E_NONFINITE",
              "E_PACKET_BUDGET",
              "UNUSED_VERTEX_NONFINITE"
            ]
          },
          "fieldPath": {
            "type": "string",
            "minLength": 1
          },
          "sourceIndices": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            }
          },
          "detail": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "code",
          "fieldPath",
          "sourceIndices",
          "detail"
        ],
        "additionalProperties": false
      },
      "RenderNativeIsosurfacePacketSuccess": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "success"
          },
          "namespace": {
            "const": "vestacraft.render.native-isosurface/1"
          },
          "stage": {
            "const": "native-post-camera-sort-surface-packet"
          },
          "sourceMeshParameterDigest": {
            "$ref": "#/$defs/Digest"
          },
          "ownerId": {
            "type": "string",
            "minLength": 1
          },
          "revision": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "surfaceVisible": {
            "type": "boolean"
          },
          "vertexNormalViewZF32Bits": {
            "type": "array",
            "items": {
              "oneOf": [
                {
                  "$ref": "#/$defs/F32Bits"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "vertexRGBAU8": {
            "type": "array",
            "items": {
              "oneOf": [
                {
                  "$ref": "#/$defs/RGBAU8"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "sourceTriangleOrdinalsInDrawOrder": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            }
          },
          "triangleIndicesInDrawOrder": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Triangle"
            }
          },
          "palette": {
            "oneOf": [
              {
                "$ref": "#/$defs/OwnedPalette"
              },
              {
                "type": "null"
              }
            ]
          },
          "material": {
            "$ref": "#/$defs/SurfaceMaterialPlan"
          },
          "filledBoundaryPasses": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/BoundaryPass"
            }
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PacketDiagnostic"
            }
          },
          "parameterDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "kind",
          "namespace",
          "stage",
          "sourceMeshParameterDigest",
          "ownerId",
          "revision",
          "surfaceVisible",
          "vertexNormalViewZF32Bits",
          "vertexRGBAU8",
          "sourceTriangleOrdinalsInDrawOrder",
          "triangleIndicesInDrawOrder",
          "palette",
          "material",
          "filledBoundaryPasses",
          "diagnostics",
          "parameterDigest"
        ],
        "additionalProperties": false,
        "x-invariants": [
          "draworderisstableperactualf32depthsum/sourcecomparators,notrandom triangleorder",
          "rgbaByVertexlength==premeshvertices;onlyunusedentriesmaynull,allusedindicesRGBAfiniteexactU8",
          "normalViewZF32derivedfromSceneMthirdrowf32;alphausesnormalfacingnotvertexdepth",
          "filledboundaryplansfollowactualsourcepasses;contoursseparateboundproducer/framepacket;nofakewholevisualcompleteclaimfromsurfacepacketalone",
          "nativeVRMLexportflattening/6decimalgeometryandSTLsourcewinding/caplossrules inrender-consumption.contract.json;notgenericpacketroundtrip"
        ]
      },
      "RenderNativeIsosurfacePacketFailure": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "failure"
          },
          "namespace": {
            "const": "vestacraft.render.native-isosurface/1"
          },
          "diagnostics": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PacketDiagnostic"
            }
          },
          "sourceMeshParameterDigest": {
            "$ref": "#/$defs/Digest"
          },
          "ownerId": {
            "type": "string",
            "minLength": 1
          },
          "revision": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "kind",
          "namespace",
          "diagnostics",
          "sourceMeshParameterDigest",
          "ownerId",
          "revision"
        ],
        "additionalProperties": false
      },
      "RenderNativeIsosurfacePacketResponse": {
        "oneOf": [
          {
            "$ref": "#/$defs/RenderNativeIsosurfacePacketSuccess"
          },
          {
            "$ref": "#/$defs/RenderNativeIsosurfacePacketFailure"
          }
        ]
      }
    },
    "oneOf": [
      {
        "$ref": "#/$defs/RenderNativeIsosurfacePacketRequest"
      },
      {
        "$ref": "#/$defs/RenderNativeIsosurfacePacketResponse"
      }
    ],
    "x-design-only": true,
    "x-imports": [
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/fs2-scientific-successor.schema.json",
        "sha256": "06e893095ff26d40a8cc379730591e7f4cdb7c86d09a7d78f1f32eee1617e63a",
        "pointer": "#/$defs/NativeMeshOutput",
        "inlineRoot": "#/$defs/FS_NativeMeshOutput",
        "definitions": [
          "FS_F32Bits",
          "FS_FC_NativeLayerCurrent1",
          "FS_FC_NativeStoredScalarIsoview1",
          "FS_FS1_Digest",
          "FS_FS1_Id",
          "FS_NativeMeshBranchRange",
          "FS_NativeMeshOutput",
          "FS_NativeMeshTriangle",
          "FS_Vec3F32Bits"
        ],
        "definitionMapping": [
          {
            "sourcePointer": "#/$defs/NativeMeshOutput",
            "inlinedPointer": "#/$defs/FS_NativeMeshOutput"
          },
          {
            "sourcePointer": "#/$defs/NativeMeshBranchRange",
            "inlinedPointer": "#/$defs/FS_NativeMeshBranchRange"
          },
          {
            "sourcePointer": "#/$defs/FS1_Id",
            "inlinedPointer": "#/$defs/FS_FS1_Id"
          },
          {
            "sourcePointer": "#/$defs/F32Bits",
            "inlinedPointer": "#/$defs/FS_F32Bits"
          },
          {
            "sourcePointer": "#/$defs/FS1_Digest",
            "inlinedPointer": "#/$defs/FS_FS1_Digest"
          },
          {
            "sourcePointer": "#/$defs/FC_NativeLayerCurrent1",
            "inlinedPointer": "#/$defs/FS_FC_NativeLayerCurrent1"
          },
          {
            "sourcePointer": "#/$defs/FC_NativeStoredScalarIsoview1",
            "inlinedPointer": "#/$defs/FS_FC_NativeStoredScalarIsoview1"
          },
          {
            "sourcePointer": "#/$defs/Vec3F32Bits",
            "inlinedPointer": "#/$defs/FS_Vec3F32Bits"
          },
          {
            "sourcePointer": "#/$defs/NativeMeshTriangle",
            "inlinedPointer": "#/$defs/FS_NativeMeshTriangle"
          }
        ],
        "method": "exactshapeinline,onlyrefsresolvedandprefixrenamed"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json",
        "sha256": "adf539aac53ddd41b46651886b00e9d5394a96c36b83f74aeb6e35984898000a",
        "pointer": "exactreachabledefs",
        "inlineRoot": "exactreachabledefs",
        "definitions": [
          "IM2_1_Digest",
          "IM2_1_Id"
        ],
        "definitionMapping": [
          {
            "sourcePointer": "#/$defs/Id",
            "inlinedPointer": "#/$defs/IM2_1_Id"
          },
          {
            "sourcePointer": "#/$defs/Digest",
            "inlinedPointer": "#/$defs/IM2_1_Digest"
          }
        ],
        "method": "exactshapeinline,onlyrefsresolvedandprefixrenamed"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-layer-current.schema.json",
        "sha256": "31e8aa75b924339c9168994f7a8615b4540a52c3294c07a694613a66874a42de",
        "pointer": "exactreachabledefs",
        "inlineRoot": "exactreachabledefs",
        "definitions": [
          "IM7_2_ROOT"
        ],
        "definitionMapping": [
          {
            "sourcePointer": "#",
            "inlinedPointer": "#/$defs/IM7_2_ROOT"
          }
        ],
        "method": "exactshapeinline,onlyrefsresolvedandprefixrenamed"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-stored-isoview.schema.json",
        "sha256": "5cdc2d8ddd4c1f146c984b61bc2a4f869d96d5411f870e788aa61cd4dc6387db",
        "pointer": "exactreachabledefs",
        "inlineRoot": "exactreachabledefs",
        "definitions": [
          "IM9_3_ROOT"
        ],
        "definitionMapping": [
          {
            "sourcePointer": "#",
            "inlinedPointer": "#/$defs/IM9_3_ROOT"
          }
        ],
        "method": "exactshapeinline,onlyrefsresolvedandprefixrenamed"
      },
      {
        "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/sg/sg2.schema.json",
        "sha256": "5d24de755b9e937a6d0e18f02ad967889e8a8cf7221cad8c41b326c97647f688",
        "pointer": "#/$defs/NativeCamera",
        "inlineRoot": "#/$defs/SG_NativeCamera",
        "definitions": [
          "SG_Matrix4Rows",
          "SG_NativeCamera",
          "SG_Vec4"
        ],
        "definitionMapping": [
          {
            "sourcePointer": "#/$defs/NativeCamera",
            "inlinedPointer": "#/$defs/SG_NativeCamera"
          },
          {
            "sourcePointer": "#/$defs/Matrix4Rows",
            "inlinedPointer": "#/$defs/SG_Matrix4Rows"
          },
          {
            "sourcePointer": "#/$defs/Vec4",
            "inlinedPointer": "#/$defs/SG_Vec4"
          }
        ],
        "method": "exactshapeinline,onlyrefsresolvedandprefixrenamed"
      }
    ],
    "x-rules": {
      "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/render-consumption/render-consumption.contract.json",
      "sha256": "7fc4cbf807b4ef2ed2ad429b957b9ceaa399a40e2f798c5ee041d73a86daed7c"
    },
    "x-old-registered177-changed": false
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/application/preferences-composition.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:preference-composition:1",
    "title": "P1-06 complete twelve-field transport composition proposal, design only; nativequalification unchanged",
    "$defs": {
      "Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$"
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "U64": {
        "type": "string",
        "pattern": "^(0|[1-9][0-9]*)$",
        "maxLength": 20
      },
      "ArtifactChoice": {
        "type": "object",
        "properties": {
          "artifactId": {
            "type": "string",
            "minLength": 1,
            "maxLength": 256
          },
          "manifestDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          }
        },
        "required": [
          "artifactId",
          "manifestDigest"
        ],
        "additionalProperties": false
      },
      "FontChoice": {
        "type": "object",
        "properties": {
          "fontResourceDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "faceIndex": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "sizePt": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "fontCapabilityDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          }
        },
        "required": [
          "fontResourceDigest",
          "faceIndex",
          "sizePt",
          "fontCapabilityDigest"
        ],
        "additionalProperties": false
      },
      "NumericDraft": {
        "type": "object",
        "properties": {
          "rawText": {
            "type": "string",
            "maxLength": 4096
          },
          "domainProfileDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "parseProfileDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          }
        },
        "required": [
          "rawText",
          "domainProfileDigest",
          "parseProfileDigest"
        ],
        "additionalProperties": false
      },
      "PreferenceChange": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "rietanArtifact"
              },
              "value": {
                "oneOf": [
                  {
                    "$ref": "#/$defs/ArtifactChoice"
                  },
                  {
                    "type": "null"
                  }
                ]
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "viewerArtifact"
              },
              "value": {
                "oneOf": [
                  {
                    "$ref": "#/$defs/ArtifactChoice"
                  },
                  {
                    "type": "null"
                  }
                ]
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "templateArtifact"
              },
              "value": {
                "oneOf": [
                  {
                    "$ref": "#/$defs/ArtifactChoice"
                  },
                  {
                    "type": "null"
                  }
                ]
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "textFont"
              },
              "value": {
                "$ref": "#/$defs/FontChoice"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "openTarget"
              },
              "value": {
                "enum": [
                  "newTab",
                  "currentTab"
                ]
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "animationStepDeg"
              },
              "value": {
                "$ref": "#/$defs/NumericDraft"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "animationIntervalMs"
              },
              "value": {
                "$ref": "#/$defs/NumericDraft"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "startupBondSearch"
              },
              "value": {
                "type": "boolean"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "hardwareAntialias"
              },
              "value": {
                "type": "boolean"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "rasterTessellationFactor"
              },
              "value": {
                "$ref": "#/$defs/NumericDraft"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "isosurfaceN"
              },
              "value": {
                "$ref": "#/$defs/NumericDraft"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "field": {
                "const": "historyLimit"
              },
              "value": {
                "$ref": "#/$defs/NumericDraft"
              }
            },
            "required": [
              "field",
              "value"
            ],
            "additionalProperties": false
          }
        ]
      },
      "ValidateRequest": {
        "type": "object",
        "properties": {
          "schema": {
            "const": "vestacraft.preference-composition/1"
          },
          "requestId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "basePreferenceVersion": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "baseEffectiveDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "ownerProfileDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "op": {
            "const": "validate"
          },
          "changes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/PreferenceChange"
            },
            "minItems": 1,
            "maxItems": 12
          }
        },
        "required": [
          "schema",
          "requestId",
          "basePreferenceVersion",
          "baseEffectiveDigest",
          "ownerProfileDigest",
          "op",
          "changes"
        ],
        "additionalProperties": false
      },
      "ApplyRequest": {
        "type": "object",
        "properties": {
          "schema": {
            "const": "vestacraft.preference-composition/1"
          },
          "requestId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "basePreferenceVersion": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "baseEffectiveDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "ownerProfileDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "op": {
            "const": "apply"
          },
          "validatedPlanDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "trimConsent": {
            "oneOf": [
              {
                "type": "object",
                "properties": {
                  "lostSnapshotIds": {
                    "type": "array",
                    "items": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$"
                    },
                    "minItems": 1
                  },
                  "planDigest": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{64}$"
                  },
                  "explicit": {
                    "const": true
                  }
                },
                "required": [
                  "lostSnapshotIds",
                  "planDigest",
                  "explicit"
                ],
                "additionalProperties": false
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "schema",
          "requestId",
          "basePreferenceVersion",
          "baseEffectiveDigest",
          "ownerProfileDigest",
          "op",
          "validatedPlanDigest",
          "trimConsent"
        ],
        "additionalProperties": false
      },
      "QueryRequest": {
        "type": "object",
        "properties": {
          "schema": {
            "const": "vestacraft.preference-composition/1"
          },
          "requestId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "op": {
            "const": "query-same-apply"
          }
        },
        "required": [
          "schema",
          "requestId",
          "op"
        ],
        "additionalProperties": false
      },
      "Request": {
        "oneOf": [
          {
            "$ref": "#/$defs/ValidateRequest"
          },
          {
            "$ref": "#/$defs/ApplyRequest"
          },
          {
            "$ref": "#/$defs/QueryRequest"
          }
        ]
      },
      "AppliedReply": {
        "type": "object",
        "properties": {
          "schema": {
            "const": "vestacraft.preference-composition/1"
          },
          "requestId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "outcome": {
            "const": "applied"
          },
          "effectivePreferenceVersion": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "actualGenerationDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "installedEffectiveDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "changedFields": {
            "type": "array",
            "items": {
              "enum": [
                "rietanArtifact",
                "viewerArtifact",
                "templateArtifact",
                "textFont",
                "openTarget",
                "animationStepDeg",
                "animationIntervalMs",
                "startupBondSearch",
                "hardwareAntialias",
                "rasterTessellationFactor",
                "isosurfaceN",
                "historyLimit"
              ]
            },
            "minItems": 1,
            "maxItems": 12
          },
          "currentDocumentMutated": {
            "const": false
          }
        },
        "required": [
          "schema",
          "requestId",
          "outcome",
          "effectivePreferenceVersion",
          "actualGenerationDigest",
          "installedEffectiveDigest",
          "changedFields",
          "currentDocumentMutated"
        ],
        "additionalProperties": false
      },
      "UnknownReply": {
        "type": "object",
        "properties": {
          "schema": {
            "const": "vestacraft.preference-composition/1"
          },
          "requestId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "outcome": {
            "const": "committed-unverified-preferences"
          },
          "attemptedGenerationDigest": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "effectivePreferenceVersionBefore": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          },
          "retryAction": {
            "const": "query-same-apply"
          }
        },
        "required": [
          "schema",
          "requestId",
          "outcome",
          "attemptedGenerationDigest",
          "effectivePreferenceVersionBefore",
          "retryAction"
        ],
        "additionalProperties": false
      },
      "Reply": {
        "oneOf": [
          {
            "$ref": "#/$defs/AppliedReply"
          },
          {
            "$ref": "#/$defs/UnknownReply"
          }
        ]
      }
    },
    "$ref": "#/$defs/Request"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/external/external-workflow-final.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:external-workflow:3",
    "title": "EX/3 source-closed four workflow successor, final revision3",
    "description": "Closed exposed workflow parameters. Tool/native scientific dialect and typed candidate/source validation remain separate, version-pinned semantic requirements. No free options JSON or paths.",
    "$defs": {
      "Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$"
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "U64": {
        "type": "string",
        "pattern": "^(0|[1-9][0-9]*)$",
        "maxLength": 20
      },
      "ResourceRef": {
        "type": "object",
        "properties": {
          "sha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "byteLength": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$",
            "maxLength": 20
          }
        },
        "required": [
          "sha256",
          "byteLength"
        ],
        "additionalProperties": false
      },
      "LossConsent": {
        "oneOf": [
          {
            "type": "null"
          },
          {
            "type": "object",
            "properties": {
              "sourceDigest": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "profileId": {
                "type": "string"
              },
              "lossReportSha256": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "explicit": {
                "const": true
              }
            },
            "required": [
              "sourceDigest",
              "profileId",
              "lossReportSha256",
              "explicit"
            ],
            "additionalProperties": false
          }
        ]
      },
      "Request": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "schema": {
                "const": "vestacraft.external-workflow/3"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "expectedRevision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "operation": {
                "const": "tidy.standardize"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  }
                },
                "required": [
                  "phaseId"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "schema",
              "requestId",
              "sessionId",
              "documentId",
              "epoch",
              "expectedRevision",
              "operation",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schema": {
                "const": "vestacraft.external-workflow/3"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "expectedRevision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "operation": {
                "const": "madel.calculate"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "radiusA": {
                    "type": "number",
                    "exclusiveMinimum": 0
                  },
                  "regionInvA": {
                    "type": "number",
                    "exclusiveMinimum": 0
                  },
                  "lossConsent": {
                    "$ref": "#/$defs/LossConsent"
                  }
                },
                "required": [
                  "phaseId",
                  "radiusA",
                  "regionInvA",
                  "lossConsent"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "schema",
              "requestId",
              "sessionId",
              "documentId",
              "epoch",
              "expectedRevision",
              "operation",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schema": {
                "const": "vestacraft.external-workflow/3"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "expectedRevision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "operation": {
                "const": "rietan.calculate"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "template": {
                    "$ref": "#/$defs/ResourceRef"
                  },
                  "companions": {
                    "type": "array",
                    "items": {
                      "type": "object",
                      "properties": {
                        "role": {
                          "enum": [
                            "int",
                            "bkg",
                            "ffe",
                            "fba"
                          ]
                        },
                        "resource": {
                          "$ref": "#/$defs/ResourceRef"
                        }
                      },
                      "required": [
                        "role",
                        "resource"
                      ],
                      "additionalProperties": false
                    },
                    "maxItems": 4
                  },
                  "lossConsent": {
                    "$ref": "#/$defs/LossConsent"
                  }
                },
                "required": [
                  "phaseId",
                  "template",
                  "companions",
                  "lossConsent"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "schema",
              "requestId",
              "sessionId",
              "documentId",
              "epoch",
              "expectedRevision",
              "operation",
              "params"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "schema": {
                "const": "vestacraft.external-workflow/3"
              },
              "requestId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "sessionId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "documentId": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "epoch": {
                "type": "string",
                "pattern": "^[0-9a-f]{32}$"
              },
              "expectedRevision": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$",
                "maxLength": 20
              },
              "operation": {
                "const": "orffe.attach-result"
              },
              "params": {
                "type": "object",
                "properties": {
                  "phaseId": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{32}$"
                  },
                  "resource": {
                    "$ref": "#/$defs/ResourceRef"
                  }
                },
                "required": [
                  "phaseId",
                  "resource"
                ],
                "additionalProperties": false
              }
            },
            "required": [
              "schema",
              "requestId",
              "sessionId",
              "documentId",
              "epoch",
              "expectedRevision",
              "operation",
              "params"
            ],
            "additionalProperties": false
          }
        ]
      },
      "MadelNumericInterpretation": {
        "type": "object",
        "properties": {
          "producerParserDigest": {
            "$ref": "#/$defs/Digest"
          },
          "inputDigest": {
            "$ref": "#/$defs/Digest"
          },
          "potentialUnit": {
            "const": "e/angstrom"
          },
          "energyUnit": {
            "const": "e^2/angstrom"
          },
          "energyAsymmetricDecimal": {
            "type": "string",
            "pattern": "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$"
          },
          "generalMultiplicity": {
            "$ref": "#/$defs/U64"
          },
          "energyCellDecimal": {
            "type": "string",
            "pattern": "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$"
          },
          "formulaUnitsPerCell": {
            "oneOf": [
              {
                "type": "null"
              },
              {
                "$ref": "#/$defs/U64"
              }
            ]
          },
          "energyPerFormulaDecimal": {
            "oneOf": [
              {
                "type": "null"
              },
              {
                "type": "string",
                "pattern": "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$"
              }
            ]
          },
          "sitePotentials": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "emittedRow": {
                  "$ref": "#/$defs/U64"
                },
                "sourceSiteMapDigest": {
                  "$ref": "#/$defs/Digest"
                },
                "potentialDecimal": {
                  "type": "string",
                  "pattern": "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$"
                }
              },
              "required": [
                "emittedRow",
                "sourceSiteMapDigest",
                "potentialDecimal"
              ],
              "additionalProperties": false
            },
            "maxItems": 300,
            "minItems": 1
          },
          "numericAcceptanceReceipt": {
            "oneOf": [
              {
                "type": "null"
              },
              {
                "$ref": "#/$defs/ResourceRef"
              }
            ]
          },
          "energyEVAsymmetricDecimal": {
            "type": "string",
            "pattern": "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$"
          },
          "energyMJmolAsymmetricDecimal": {
            "type": "string",
            "pattern": "^[+-]?(?:[0-9]+(?:\\.[0-9]*)?|\\.[0-9]+)(?:[Ee][+-]?[0-9]+)?$"
          },
          "reportProfileDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "producerParserDigest",
          "inputDigest",
          "potentialUnit",
          "energyUnit",
          "energyAsymmetricDecimal",
          "generalMultiplicity",
          "energyCellDecimal",
          "formulaUnitsPerCell",
          "energyPerFormulaDecimal",
          "sitePotentials",
          "numericAcceptanceReceipt",
          "energyEVAsymmetricDecimal",
          "energyMJmolAsymmetricDecimal",
          "reportProfileDigest"
        ],
        "additionalProperties": false
      },
      "TemplateRevision": {
        "type": "object",
        "properties": {
          "originalTemplate": {
            "$ref": "#/$defs/ResourceRef"
          },
          "preparedInputIns": {
            "$ref": "#/$defs/ResourceRef"
          },
          "returnedIns": {
            "$ref": "#/$defs/ResourceRef"
          },
          "sourcePhaseDigest": {
            "$ref": "#/$defs/Digest"
          },
          "preparedPhaseDigest": {
            "$ref": "#/$defs/Digest"
          },
          "sourceMap": {
            "$ref": "#/$defs/ResourceRef"
          },
          "patchLedger": {
            "$ref": "#/$defs/ResourceRef"
          },
          "controlProfileDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "originalTemplate",
          "preparedInputIns",
          "returnedIns",
          "sourcePhaseDigest",
          "preparedPhaseDigest",
          "sourceMap",
          "patchLedger",
          "controlProfileDigest"
        ],
        "additionalProperties": false
      },
      "WorkflowResult": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "tidy.standardize"
              },
              "jobId": {
                "$ref": "#/$defs/Id"
              },
              "sessionId": {
                "$ref": "#/$defs/Id"
              },
              "documentId": {
                "$ref": "#/$defs/Id"
              },
              "epoch": {
                "$ref": "#/$defs/Id"
              },
              "baseRevision": {
                "$ref": "#/$defs/U64"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "inputManifestDigest": {
                "$ref": "#/$defs/Digest"
              },
              "workflowProfileDigest": {
                "$ref": "#/$defs/Digest"
              },
              "kind": {
                "const": "phase-candidate"
              },
              "report": {
                "$ref": "#/$defs/ResourceRef"
              },
              "candidatePhase": {
                "$ref": "#/$defs/ResourceRef"
              },
              "sourceToResultMap": {
                "$ref": "#/$defs/ResourceRef"
              },
              "validationReceipt": {
                "$ref": "#/$defs/ResourceRef"
              }
            },
            "required": [
              "operation",
              "jobId",
              "sessionId",
              "documentId",
              "epoch",
              "baseRevision",
              "phaseId",
              "inputManifestDigest",
              "workflowProfileDigest",
              "kind",
              "report",
              "candidatePhase",
              "sourceToResultMap",
              "validationReceipt"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "rietan.calculate"
              },
              "jobId": {
                "$ref": "#/$defs/Id"
              },
              "sessionId": {
                "$ref": "#/$defs/Id"
              },
              "documentId": {
                "$ref": "#/$defs/Id"
              },
              "epoch": {
                "$ref": "#/$defs/Id"
              },
              "baseRevision": {
                "$ref": "#/$defs/U64"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "inputManifestDigest": {
                "$ref": "#/$defs/Digest"
              },
              "workflowProfileDigest": {
                "$ref": "#/$defs/Digest"
              },
              "kind": {
                "const": "simulation-artifacts"
              },
              "report": {
                "$ref": "#/$defs/ResourceRef"
              },
              "templateRevision": {
                "$ref": "#/$defs/TemplateRevision"
              },
              "plotDialect": {
                "enum": [
                  "gnuplot",
                  "igor-itx"
                ]
              },
              "artifacts": {
                "type": "array",
                "items": {
                  "type": "object",
                  "properties": {
                    "role": {
                      "enum": [
                        "int",
                        "bkg",
                        "itx",
                        "hkl",
                        "xyz",
                        "fos",
                        "ffe",
                        "fba",
                        "ffi",
                        "ffo",
                        "vesta",
                        "plt",
                        "gpd",
                        "alb",
                        "prf",
                        "inflip",
                        "exp",
                        "pdf"
                      ]
                    },
                    "resource": {
                      "$ref": "#/$defs/ResourceRef"
                    }
                  },
                  "required": [
                    "role",
                    "resource"
                  ],
                  "additionalProperties": false
                },
                "maxItems": 18
              },
              "plotOutcome": {
                "enum": [
                  "ready",
                  "failed",
                  "not-requested"
                ]
              },
              "numericAcceptanceReceipt": {
                "$ref": "#/$defs/ResourceRef"
              }
            },
            "required": [
              "operation",
              "jobId",
              "sessionId",
              "documentId",
              "epoch",
              "baseRevision",
              "phaseId",
              "inputManifestDigest",
              "workflowProfileDigest",
              "kind",
              "report",
              "templateRevision",
              "plotDialect",
              "artifacts",
              "plotOutcome",
              "numericAcceptanceReceipt"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "orffe.attach-result"
              },
              "jobId": {
                "$ref": "#/$defs/Id"
              },
              "sessionId": {
                "$ref": "#/$defs/Id"
              },
              "documentId": {
                "$ref": "#/$defs/Id"
              },
              "epoch": {
                "$ref": "#/$defs/Id"
              },
              "baseRevision": {
                "$ref": "#/$defs/U64"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "inputManifestDigest": {
                "$ref": "#/$defs/Digest"
              },
              "workflowProfileDigest": {
                "$ref": "#/$defs/Digest"
              },
              "kind": {
                "const": "resource-table-candidate"
              },
              "sourceResource": {
                "$ref": "#/$defs/ResourceRef"
              },
              "sourceTable": {
                "$ref": "#/$defs/ResourceRef"
              },
              "activeMapping": {
                "$ref": "#/$defs/ResourceRef"
              },
              "validationReceipt": {
                "$ref": "#/$defs/ResourceRef"
              }
            },
            "required": [
              "operation",
              "jobId",
              "sessionId",
              "documentId",
              "epoch",
              "baseRevision",
              "phaseId",
              "inputManifestDigest",
              "workflowProfileDigest",
              "kind",
              "sourceResource",
              "sourceTable",
              "activeMapping",
              "validationReceipt"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "madel.calculate"
              },
              "jobId": {
                "$ref": "#/$defs/Id"
              },
              "sessionId": {
                "$ref": "#/$defs/Id"
              },
              "documentId": {
                "$ref": "#/$defs/Id"
              },
              "epoch": {
                "$ref": "#/$defs/Id"
              },
              "baseRevision": {
                "$ref": "#/$defs/U64"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "inputManifestDigest": {
                "$ref": "#/$defs/Digest"
              },
              "workflowProfileDigest": {
                "$ref": "#/$defs/Digest"
              },
              "kind": {
                "const": "diagnostic-report"
              },
              "report": {
                "$ref": "#/$defs/ResourceRef"
              },
              "diagnostics": {
                "$ref": "#/$defs/ResourceRef"
              },
              "numericInterpretation": {
                "type": "null"
              },
              "convergenceState": {
                "enum": [
                  "unassessed"
                ]
              }
            },
            "required": [
              "operation",
              "jobId",
              "sessionId",
              "documentId",
              "epoch",
              "baseRevision",
              "phaseId",
              "inputManifestDigest",
              "workflowProfileDigest",
              "kind",
              "report",
              "diagnostics",
              "numericInterpretation",
              "convergenceState"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "madel.calculate"
              },
              "jobId": {
                "$ref": "#/$defs/Id"
              },
              "sessionId": {
                "$ref": "#/$defs/Id"
              },
              "documentId": {
                "$ref": "#/$defs/Id"
              },
              "epoch": {
                "$ref": "#/$defs/Id"
              },
              "baseRevision": {
                "$ref": "#/$defs/U64"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "inputManifestDigest": {
                "$ref": "#/$defs/Digest"
              },
              "workflowProfileDigest": {
                "$ref": "#/$defs/Digest"
              },
              "kind": {
                "const": "finite-calculation"
              },
              "report": {
                "$ref": "#/$defs/ResourceRef"
              },
              "diagnostics": {
                "$ref": "#/$defs/ResourceRef"
              },
              "numericInterpretation": {
                "$ref": "#/$defs/MadelNumericInterpretation"
              },
              "convergenceState": {
                "enum": [
                  "unassessed",
                  "failed"
                ]
              }
            },
            "required": [
              "operation",
              "jobId",
              "sessionId",
              "documentId",
              "epoch",
              "baseRevision",
              "phaseId",
              "inputManifestDigest",
              "workflowProfileDigest",
              "kind",
              "report",
              "diagnostics",
              "numericInterpretation",
              "convergenceState"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "operation": {
                "const": "madel.calculate"
              },
              "jobId": {
                "$ref": "#/$defs/Id"
              },
              "sessionId": {
                "$ref": "#/$defs/Id"
              },
              "documentId": {
                "$ref": "#/$defs/Id"
              },
              "epoch": {
                "$ref": "#/$defs/Id"
              },
              "baseRevision": {
                "$ref": "#/$defs/U64"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "inputManifestDigest": {
                "$ref": "#/$defs/Digest"
              },
              "workflowProfileDigest": {
                "$ref": "#/$defs/Digest"
              },
              "kind": {
                "const": "qualified-report"
              },
              "report": {
                "$ref": "#/$defs/ResourceRef"
              },
              "diagnostics": {
                "$ref": "#/$defs/ResourceRef"
              },
              "numericInterpretation": {
                "allOf": [
                  {
                    "$ref": "#/$defs/MadelNumericInterpretation"
                  },
                  {
                    "type": "object",
                    "properties": {
                      "numericAcceptanceReceipt": {
                        "$ref": "#/$defs/ResourceRef"
                      }
                    },
                    "required": [
                      "numericAcceptanceReceipt"
                    ]
                  }
                ]
              },
              "convergenceState": {
                "enum": [
                  "accepted"
                ]
              }
            },
            "required": [
              "operation",
              "jobId",
              "sessionId",
              "documentId",
              "epoch",
              "baseRevision",
              "phaseId",
              "inputManifestDigest",
              "workflowProfileDigest",
              "kind",
              "report",
              "diagnostics",
              "numericInterpretation",
              "convergenceState"
            ],
            "additionalProperties": false
          }
        ],
        "description": "Server-produced result descriptor, never a caller-supplied success declaration. Resource, identity, numeric and receipt semantic assertions are mandatory in addition to shape."
      }
    },
    "$ref": "#/$defs/Request"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:design:field-science:1",
    "title": "P1-02 FS/1 design profile, partial",
    "$defs": {
      "Id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "title": "Id"
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$",
        "title": "Digest"
      },
      "Vec3": {
        "type": "array",
        "items": {
          "type": "number"
        },
        "minItems": 3,
        "maxItems": 3,
        "title": "Vec3"
      },
      "Int3": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": -2147483648,
          "maximum": 2147483647
        },
        "minItems": 3,
        "maxItems": 3,
        "title": "Int3"
      },
      "Mat3": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Vec3"
        },
        "minItems": 3,
        "maxItems": 3,
        "title": "Mat3"
      },
      "Resource": {
        "type": "object",
        "properties": {
          "digest": {
            "$ref": "#/$defs/Digest"
          },
          "bytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "digest",
          "bytes"
        ],
        "additionalProperties": false,
        "title": "Resource"
      },
      "Species": {
        "type": "object",
        "properties": {
          "element": {
            "oneOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "charge": {
            "oneOf": [
              {
                "type": "integer",
                "minimum": -128,
                "maximum": 127
              },
              {
                "type": "null"
              }
            ]
          },
          "massNumber": {
            "oneOf": [
              {
                "type": "integer",
                "minimum": 1,
                "maximum": 65535
              },
              {
                "type": "null"
              }
            ]
          },
          "rawToken": {
            "type": "string"
          }
        },
        "required": [
          "element",
          "charge",
          "massNumber",
          "rawToken"
        ],
        "additionalProperties": false,
        "title": "Species"
      },
      "Cell": {
        "type": "object",
        "properties": {
          "lengthA": {
            "type": "array",
            "items": {
              "type": "number",
              "exclusiveMinimum": 0
            },
            "minItems": 3,
            "maxItems": 3
          },
          "anglesDeg": {
            "type": "array",
            "items": {
              "type": "number",
              "exclusiveMinimum": 0,
              "exclusiveMaximum": 180
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "lengthA",
          "anglesDeg"
        ],
        "additionalProperties": false,
        "title": "Cell"
      },
      "Adp": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "iso-U"
              },
              "uA2": {
                "type": "number"
              }
            },
            "required": [
              "kind",
              "uA2"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "tensor"
              },
              "basis": {
                "enum": [
                  "cif-U-A2",
                  "cartesian-U-A2",
                  "beta-dimensionless"
                ]
              },
              "six": {
                "type": "array",
                "items": {
                  "type": "number"
                },
                "minItems": 6,
                "maxItems": 6
              }
            },
            "required": [
              "kind",
              "basis",
              "six"
            ],
            "additionalProperties": false
          }
        ],
        "title": "Adp"
      },
      "Parameter": {
        "type": "object",
        "properties": {
          "object": {
            "$ref": "#/$defs/Id"
          },
          "field": {
            "enum": [
              "fractional.x",
              "fractional.y",
              "fractional.z",
              "cell.a",
              "cell.b",
              "cell.c",
              "cell.alpha",
              "cell.beta",
              "cell.gamma",
              "occupancy",
              "U11",
              "U22",
              "U33",
              "U12",
              "U13",
              "U23"
            ]
          },
          "unit": {
            "enum": [
              "one",
              "A",
              "degree",
              "A2"
            ]
          }
        },
        "required": [
          "object",
          "field",
          "unit"
        ],
        "additionalProperties": false,
        "title": "Parameter"
      },
      "Covariance": {
        "type": "object",
        "properties": {
          "parameters": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Parameter"
            }
          },
          "matrix": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "source": {
            "enum": [
              "supplied-full",
              "supplied-diagonal"
            ]
          },
          "sourceDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "parameters",
          "matrix",
          "source",
          "sourceDigest"
        ],
        "additionalProperties": false,
        "title": "Covariance"
      },
      "Site": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "label": {
            "type": "string"
          },
          "species": {
            "$ref": "#/$defs/Species"
          },
          "fractional": {
            "$ref": "#/$defs/Vec3"
          },
          "occupancy": {
            "type": "number"
          },
          "adp": {
            "oneOf": [
              {
                "$ref": "#/$defs/Adp"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "id",
          "label",
          "species",
          "fractional",
          "occupancy",
          "adp"
        ],
        "additionalProperties": false,
        "title": "Site"
      },
      "SymmetryOp": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "matrix": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Int3"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "translation": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "numerator": {
                  "type": "integer"
                },
                "denominator": {
                  "type": "integer",
                  "minimum": 1
                }
              },
              "required": [
                "numerator",
                "denominator"
              ],
              "additionalProperties": false
            },
            "minItems": 3,
            "maxItems": 3
          },
          "timeReversal": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "id",
          "matrix",
          "translation",
          "timeReversal"
        ],
        "additionalProperties": false,
        "title": "SymmetryOp"
      },
      "Structure": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Site"
            }
          },
          "operations": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SymmetryOp"
            }
          },
          "covariance": {
            "oneOf": [
              {
                "$ref": "#/$defs/Covariance"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "cell",
          "sites",
          "operations",
          "covariance"
        ],
        "additionalProperties": false,
        "title": "Structure"
      },
      "Placement": {
        "type": "object",
        "properties": {
          "phase": {
            "$ref": "#/$defs/Id"
          },
          "parent": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "rotation": {
            "$ref": "#/$defs/Mat3"
          },
          "translationA": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "phase",
          "parent",
          "rotation",
          "translationA"
        ],
        "additionalProperties": false,
        "title": "Placement"
      },
      "Image": {
        "type": "object",
        "properties": {
          "phase": {
            "$ref": "#/$defs/Id"
          },
          "site": {
            "$ref": "#/$defs/Id"
          },
          "operation": {
            "$ref": "#/$defs/Id"
          },
          "shift": {
            "$ref": "#/$defs/Int3"
          }
        },
        "required": [
          "phase",
          "site",
          "operation",
          "shift"
        ],
        "additionalProperties": false,
        "title": "Image"
      },
      "ImagePoint": {
        "type": "object",
        "properties": {
          "image": {
            "$ref": "#/$defs/Image"
          },
          "cartesianA": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "image",
          "cartesianA"
        ],
        "additionalProperties": false,
        "title": "ImagePoint"
      },
      "Field": {
        "type": "object",
        "properties": {
          "resource": {
            "$ref": "#/$defs/Resource"
          },
          "kind": {
            "enum": [
              "electronDensity",
              "nuclearDensity",
              "electronPatterson",
              "nuclearPatterson",
              "generic"
            ]
          },
          "unit": {
            "enum": [
              "ePerA3",
              "fmPerA3",
              "e2PerA3",
              "fm2PerA3",
              "declared"
            ]
          },
          "declaredUnit": {
            "oneOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "coreShape": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1
            },
            "minItems": 3,
            "maxItems": 3
          },
          "closure": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "periodic": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "axisOrder": {
            "enum": [
              "x-fastest",
              "z-fastest"
            ]
          },
          "originA": {
            "$ref": "#/$defs/Vec3"
          },
          "dtype": {
            "enum": [
              "f32",
              "f64"
            ]
          },
          "stepBasisA": {
            "$ref": "#/$defs/Mat3"
          }
        },
        "required": [
          "resource",
          "kind",
          "unit",
          "declaredUnit",
          "coreShape",
          "closure",
          "periodic",
          "axisOrder",
          "originA",
          "stepBasisA",
          "dtype"
        ],
        "additionalProperties": false,
        "title": "Field"
      },
      "Reflection": {
        "type": "object",
        "properties": {
          "hkl": {
            "$ref": "#/$defs/Int3"
          },
          "real": {
            "type": "number"
          },
          "imaginary": {
            "type": "number"
          },
          "unit": {
            "enum": [
              "electron",
              "fm"
            ]
          },
          "source": {
            "enum": [
              "Fc",
              "Fo",
              "difference"
            ]
          }
        },
        "required": [
          "hkl",
          "real",
          "imaginary",
          "unit",
          "source"
        ],
        "additionalProperties": false,
        "title": "Reflection"
      },
      "Mesh": {
        "type": "object",
        "properties": {
          "verticesA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            }
          },
          "triangles": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 4294967295
              },
              "minItems": 3,
              "maxItems": 3
            }
          },
          "normals": {
            "oneOf": [
              {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Vec3"
                }
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "verticesA",
          "triangles",
          "normals"
        ],
        "additionalProperties": false,
        "title": "Mesh"
      },
      "Camera": {
        "type": "object",
        "properties": {
          "rotation": {
            "$ref": "#/$defs/Mat3"
          },
          "projection": {
            "enum": [
              "parallel",
              "perspective"
            ]
          },
          "zoom": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "widthA": {
            "oneOf": [
              {
                "type": "number",
                "exclusiveMinimum": 0
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "rotation",
          "projection",
          "zoom",
          "widthA"
        ],
        "additionalProperties": false,
        "title": "Camera"
      },
      "Diagnostic": {
        "type": "object",
        "properties": {
          "code": {
            "type": "string"
          },
          "object": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "field": {
            "oneOf": [
              {
                "type": "string"
              },
              {
                "type": "null"
              }
            ]
          },
          "message": {
            "type": "string"
          }
        },
        "required": [
          "code",
          "object",
          "field",
          "message"
        ],
        "additionalProperties": false,
        "title": "Diagnostic"
      },
      "Plane": {
        "type": "object",
        "properties": {
          "unitNormalCartesian": {
            "$ref": "#/$defs/Vec3"
          },
          "offsetA": {
            "type": "number"
          },
          "spacingA": {
            "type": "number",
            "exclusiveMinimum": 0
          }
        },
        "required": [
          "unitNormalCartesian",
          "offsetA",
          "spacingA"
        ],
        "additionalProperties": false,
        "title": "Plane"
      },
      "Metric": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string"
          },
          "value": {
            "type": "number"
          },
          "unit": {
            "enum": [
              "A",
              "A2",
              "A3",
              "degree",
              "one",
              "e",
              "fm"
            ]
          }
        },
        "required": [
          "name",
          "value",
          "unit"
        ],
        "additionalProperties": false,
        "title": "Metric"
      },
      "Peak": {
        "type": "object",
        "properties": {
          "fractional": {
            "$ref": "#/$defs/Vec3"
          },
          "value": {
            "type": "number"
          },
          "kind": {
            "enum": [
              "maximum",
              "minimum"
            ]
          }
        },
        "required": [
          "fractional",
          "value",
          "kind"
        ],
        "additionalProperties": false,
        "title": "Peak"
      },
      "Line": {
        "type": "object",
        "properties": {
          "distanceA": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "values": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "fieldUnit": {
            "type": "string"
          }
        },
        "required": [
          "distanceA",
          "values",
          "fieldUnit"
        ],
        "additionalProperties": false,
        "title": "Line"
      },
      "Slice": {
        "type": "object",
        "properties": {
          "width": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "height": {
            "type": "integer",
            "minimum": 0,
            "maximum": 4294967295
          },
          "values": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "originA": {
            "$ref": "#/$defs/Vec3"
          },
          "uA": {
            "$ref": "#/$defs/Vec3"
          },
          "vA": {
            "$ref": "#/$defs/Vec3"
          },
          "unit": {
            "type": "string"
          }
        },
        "required": [
          "width",
          "height",
          "values",
          "originA",
          "uA",
          "vA",
          "unit"
        ],
        "additionalProperties": false,
        "title": "Slice"
      },
      "Bundle": {
        "type": "object",
        "properties": {
          "primary": {
            "$ref": "#/$defs/Resource"
          },
          "dependencies": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "logicalPath": {
                  "type": "string"
                },
                "resource": {
                  "$ref": "#/$defs/Resource"
                }
              },
              "required": [
                "logicalPath",
                "resource"
              ],
              "additionalProperties": false
            }
          },
          "losses": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Diagnostic"
            }
          },
          "sourceDigest": {
            "$ref": "#/$defs/Digest"
          },
          "profileId": {
            "type": "string"
          },
          "lossReportSha256": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "primary",
          "dependencies",
          "losses",
          "sourceDigest",
          "profileId",
          "lossReportSha256"
        ],
        "additionalProperties": false,
        "title": "Bundle"
      },
      "AtomInstance": {
        "type": "object",
        "properties": {
          "image": {
            "$ref": "#/$defs/Image"
          },
          "centerA": {
            "$ref": "#/$defs/Vec3"
          },
          "radiusA": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "colorRGBA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": 0,
              "maximum": 1
            },
            "minItems": 4,
            "maxItems": 4
          }
        },
        "required": [
          "image",
          "centerA",
          "radiusA",
          "colorRGBA"
        ],
        "additionalProperties": false,
        "title": "AtomInstance"
      },
      "TransformResult": {
        "type": "object",
        "properties": {
          "structure": {
            "$ref": "#/$defs/Structure"
          },
          "oldToNew": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "old": {
                  "$ref": "#/$defs/Id"
                },
                "new": {
                  "type": "array",
                  "items": {
                    "$ref": "#/$defs/Id"
                  }
                }
              },
              "required": [
                "old",
                "new"
              ],
              "additionalProperties": false
            }
          }
        },
        "required": [
          "structure",
          "oldToNew"
        ],
        "additionalProperties": false,
        "title": "TransformResult"
      },
      "Curve": {
        "type": "object",
        "properties": {
          "x": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "y": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "xUnit": {
            "enum": [
              "degree",
              "A"
            ]
          },
          "yUnit": {
            "type": "string"
          }
        },
        "required": [
          "x",
          "y",
          "xUnit",
          "yUnit"
        ],
        "additionalProperties": false,
        "title": "Curve"
      },
      "Capability": {
        "type": "object",
        "properties": {
          "action": {
            "enum": [
              "no-action"
            ]
          },
          "referenceVersion": {
            "const": "3.90.9a-macos-arm64"
          },
          "menuId": {
            "const": 185
          }
        },
        "required": [
          "action",
          "referenceVersion",
          "menuId"
        ],
        "additionalProperties": false,
        "title": "Capability"
      },
      "ExternalFiles": {
        "type": "object",
        "properties": {
          "files": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "logicalPath": {
                  "type": "string"
                },
                "resource": {
                  "$ref": "#/$defs/Resource"
                }
              },
              "required": [
                "logicalPath",
                "resource"
              ],
              "additionalProperties": false
            }
          },
          "runAuthorized": {
            "const": false
          }
        },
        "required": [
          "files",
          "runAuthorized"
        ],
        "additionalProperties": false,
        "title": "ExternalFiles"
      },
      "PattersonCoefficient": {
        "type": "object",
        "properties": {
          "hkl": {
            "$ref": "#/$defs/Int3"
          },
          "value": {
            "type": "number"
          },
          "unit": {
            "enum": [
              "electron2",
              "fm2"
            ]
          }
        },
        "required": [
          "hkl",
          "value",
          "unit"
        ],
        "additionalProperties": false,
        "title": "PattersonCoefficient"
      },
      "ResolvePlacementsInput": {
        "type": "object",
        "properties": {
          "placements": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Placement"
            }
          },
          "root": {
            "$ref": "#/$defs/Id"
          }
        },
        "required": [
          "placements",
          "root"
        ],
        "additionalProperties": false,
        "title": "ResolvePlacementsInput"
      },
      "ResolvePlacementsOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/ResolvedPlacement"
        },
        "title": "ResolvePlacementsOutput"
      },
      "ExpandImagesInput": {
        "type": "object",
        "properties": {
          "phase": {
            "$ref": "#/$defs/Id"
          },
          "structure": {
            "$ref": "#/$defs/Structure"
          },
          "shifts": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Int3"
            }
          }
        },
        "required": [
          "phase",
          "structure",
          "shifts"
        ],
        "additionalProperties": false,
        "title": "ExpandImagesInput"
      },
      "ExpandImagesOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/ImagePoint"
        },
        "title": "ExpandImagesOutput"
      },
      "ChangeBasisInput": {
        "type": "object",
        "properties": {
          "structure": {
            "$ref": "#/$defs/Structure"
          },
          "basisP": {
            "$ref": "#/$defs/Mat3"
          },
          "originFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "mode": {
            "enum": [
              "keep-listed-sites",
              "enumerated-supercell"
            ]
          },
          "translations": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Int3"
            }
          }
        },
        "required": [
          "structure",
          "basisP",
          "originFractional",
          "mode",
          "translations"
        ],
        "additionalProperties": false,
        "title": "ChangeBasisInput"
      },
      "ChangeBasisOutput": {
        "$ref": "#/$defs/TransformResult",
        "title": "ChangeBasisOutput"
      },
      "TransportAdpInput": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "adp": {
            "$ref": "#/$defs/Adp"
          }
        },
        "required": [
          "cell",
          "adp"
        ],
        "additionalProperties": false,
        "title": "TransportAdpInput"
      },
      "TransportAdpOutput": {
        "type": "object",
        "properties": {
          "cartesianUA2": {
            "$ref": "#/$defs/Mat3"
          },
          "uEquivalentA2": {
            "type": "number"
          },
          "classification": {
            "enum": [
              "positive-definite",
              "positive-semidefinite",
              "negative-definite",
              "negative-semidefinite",
              "indefinite",
              "zero"
            ]
          },
          "inertia": {
            "type": "object",
            "properties": {
              "positive": {
                "type": "integer",
                "minimum": 0,
                "maximum": 3
              },
              "zero": {
                "type": "integer",
                "minimum": 0,
                "maximum": 3
              },
              "negative": {
                "type": "integer",
                "minimum": 0,
                "maximum": 3
              }
            },
            "required": [
              "positive",
              "zero",
              "negative"
            ],
            "additionalProperties": false
          }
        },
        "required": [
          "cartesianUA2",
          "uEquivalentA2",
          "classification",
          "inertia"
        ],
        "additionalProperties": false,
        "title": "TransportAdpOutput"
      },
      "FindDuplicatesInput": {
        "type": "object",
        "properties": {
          "points": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ImagePoint"
            }
          },
          "thresholdA": {
            "type": "number",
            "minimum": 0
          }
        },
        "required": [
          "points",
          "thresholdA"
        ],
        "additionalProperties": false,
        "title": "FindDuplicatesInput"
      },
      "FindDuplicatesOutput": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/Id"
          }
        },
        "title": "FindDuplicatesOutput"
      },
      "SelectBondPairsInput": {
        "type": "object",
        "properties": {
          "points": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ImagePoint"
            }
          },
          "minimumA": {
            "type": "number",
            "minimum": 0
          },
          "maximumA": {
            "type": "number",
            "minimum": 0
          },
          "thresholdMode": {
            "enum": [
              "legacy-f32-square",
              "explicit-f64-square"
            ]
          }
        },
        "required": [
          "points",
          "minimumA",
          "maximumA",
          "thresholdMode"
        ],
        "additionalProperties": false,
        "title": "SelectBondPairsInput"
      },
      "SelectBondPairsOutput": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "a": {
              "$ref": "#/$defs/Image"
            },
            "b": {
              "$ref": "#/$defs/Image"
            },
            "distanceA": {
              "type": "number"
            }
          },
          "required": [
            "a",
            "b",
            "distanceA"
          ],
          "additionalProperties": false
        },
        "title": "SelectBondPairsOutput"
      },
      "MeasurePolyhedronInput": {
        "type": "object",
        "properties": {
          "centerA": {
            "$ref": "#/$defs/Vec3"
          },
          "mesh": {
            "$ref": "#/$defs/Mesh"
          }
        },
        "required": [
          "centerA",
          "mesh"
        ],
        "additionalProperties": false,
        "title": "MeasurePolyhedronInput"
      },
      "MeasurePolyhedronOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Metric"
        },
        "title": "MeasurePolyhedronOutput"
      },
      "TransformVectorInput": {
        "type": "object",
        "properties": {
          "vector": {
            "$ref": "#/$defs/Vec3"
          },
          "cartesianOrthogonal": {
            "$ref": "#/$defs/Mat3"
          },
          "kind": {
            "enum": [
              "polar",
              "axial"
            ]
          },
          "timeReversal": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "vector",
          "cartesianOrthogonal",
          "kind",
          "timeReversal"
        ],
        "additionalProperties": false,
        "title": "TransformVectorInput"
      },
      "TransformVectorOutput": {
        "$ref": "#/$defs/Vec3",
        "title": "TransformVectorOutput"
      },
      "MillerPlaneInput": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "hkl": {
            "$ref": "#/$defs/Int3"
          },
          "order": {
            "type": "integer"
          }
        },
        "required": [
          "cell",
          "hkl",
          "order"
        ],
        "additionalProperties": false,
        "title": "MillerPlaneInput"
      },
      "MillerPlaneOutput": {
        "$ref": "#/$defs/Plane",
        "title": "MillerPlaneOutput"
      },
      "ClipExplicitImagesInput": {
        "type": "object",
        "properties": {
          "points": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ImagePoint"
            }
          },
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "minimumFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "maximumFractional": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "points",
          "cell",
          "minimumFractional",
          "maximumFractional"
        ],
        "additionalProperties": false,
        "title": "ClipExplicitImagesInput"
      },
      "ClipExplicitImagesOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/ImagePoint"
        },
        "title": "ClipExplicitImagesOutput"
      },
      "ComposeCameraInput": {
        "type": "object",
        "properties": {
          "camera": {
            "$ref": "#/$defs/Camera"
          },
          "xDeg": {
            "type": "number"
          },
          "yDeg": {
            "type": "number"
          },
          "zDeg": {
            "type": "number"
          }
        },
        "required": [
          "camera",
          "xDeg",
          "yDeg",
          "zDeg"
        ],
        "additionalProperties": false,
        "title": "ComposeCameraInput"
      },
      "ComposeCameraOutput": {
        "$ref": "#/$defs/Camera",
        "title": "ComposeCameraOutput"
      },
      "MeasureImagesInput": {
        "type": "object",
        "properties": {
          "points": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/MeasurementPoint"
            }
          },
          "kind": {
            "enum": [
              "distance",
              "angle",
              "unsigned-dihedral"
            ]
          },
          "covariance": {
            "oneOf": [
              {
                "$ref": "#/$defs/CartesianPointCovariance"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "points",
          "kind",
          "covariance"
        ],
        "additionalProperties": false,
        "title": "MeasureImagesInput"
      },
      "MeasureImagesOutput": {
        "type": "object",
        "properties": {
          "value": {
            "type": "number"
          },
          "unit": {
            "enum": [
              "A",
              "degree"
            ]
          },
          "variance": {
            "oneOf": [
              {
                "type": "number",
                "minimum": 0
              },
              {
                "type": "null"
              }
            ]
          },
          "sigma": {
            "oneOf": [
              {
                "type": "number",
                "minimum": 0
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "value",
          "unit",
          "variance",
          "sigma"
        ],
        "additionalProperties": false,
        "title": "MeasureImagesOutput"
      },
      "ProposeTidyResultInput": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/Structure"
          },
          "candidate": {
            "$ref": "#/$defs/Structure"
          },
          "inputDigest": {
            "$ref": "#/$defs/Digest"
          },
          "toolOutput": {
            "$ref": "#/$defs/Resource"
          }
        },
        "required": [
          "source",
          "candidate",
          "inputDigest",
          "toolOutput"
        ],
        "additionalProperties": false,
        "title": "ProposeTidyResultInput"
      },
      "ProposeTidyResultOutput": {
        "type": "object",
        "properties": {
          "candidate": {
            "$ref": "#/$defs/Structure"
          },
          "inputDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "candidate",
          "inputDigest"
        ],
        "additionalProperties": false,
        "title": "ProposeTidyResultOutput"
      },
      "PrepareMadelInputInput": {
        "type": "object",
        "properties": {
          "structure": {
            "$ref": "#/$defs/Structure"
          },
          "charges": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "site": {
                  "$ref": "#/$defs/Id"
                },
                "chargeElectron": {
                  "type": "number"
                }
              },
              "required": [
                "site",
                "chargeElectron"
              ],
              "additionalProperties": false
            }
          },
          "radiusA": {
            "type": "number",
            "exclusiveMinimum": 0
          }
        },
        "required": [
          "structure",
          "charges",
          "radiusA"
        ],
        "additionalProperties": false,
        "title": "PrepareMadelInputInput"
      },
      "PrepareMadelInputOutput": {
        "$ref": "#/$defs/ExternalFiles",
        "title": "PrepareMadelInputOutput"
      },
      "ConvolvePowderLinesInput": {
        "type": "object",
        "properties": {
          "positionsDeg": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "intensities": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "sampleDeg": {
            "type": "array",
            "items": {
              "type": "number"
            }
          },
          "fwhmDeg": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "mixing": {
            "type": "number",
            "minimum": 0,
            "maximum": 1
          },
          "compatibilityFirstPointZero": {
            "type": "boolean"
          }
        },
        "required": [
          "positionsDeg",
          "intensities",
          "sampleDeg",
          "fwhmDeg",
          "mixing",
          "compatibilityFirstPointZero"
        ],
        "additionalProperties": false,
        "title": "ConvolvePowderLinesInput"
      },
      "ConvolvePowderLinesOutput": {
        "$ref": "#/$defs/Curve",
        "title": "ConvolvePowderLinesOutput"
      },
      "SumStructureFactorsInput": {
        "type": "object",
        "properties": {
          "structure": {
            "$ref": "#/$defs/Structure"
          },
          "hkl": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Int3"
            }
          },
          "siteAmplitudes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "site": {
                  "$ref": "#/$defs/Id"
                },
                "values": {
                  "type": "array",
                  "items": {
                    "type": "number"
                  }
                }
              },
              "required": [
                "site",
                "values"
              ],
              "additionalProperties": false
            }
          },
          "unit": {
            "enum": [
              "electron",
              "fm"
            ]
          }
        },
        "required": [
          "structure",
          "hkl",
          "siteAmplitudes",
          "unit"
        ],
        "additionalProperties": false,
        "title": "SumStructureFactorsInput"
      },
      "SumStructureFactorsOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Reflection"
        },
        "title": "SumStructureFactorsOutput"
      },
      "SynthesizeFourierInput": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "reflections": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Reflection"
            }
          },
          "coreShape": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1
            },
            "minItems": 3,
            "maxItems": 3
          },
          "mode": {
            "enum": [
              "Fc",
              "Fo",
              "difference"
            ]
          },
          "includeConjugates": {
            "type": "boolean"
          },
          "dcReal": {
            "type": "number"
          }
        },
        "required": [
          "cell",
          "reflections",
          "coreShape",
          "mode",
          "includeConjugates",
          "dcReal"
        ],
        "additionalProperties": false,
        "title": "SynthesizeFourierInput"
      },
      "SynthesizeFourierOutput": {
        "$ref": "#/$defs/Field",
        "title": "SynthesizeFourierOutput"
      },
      "ElectronDcInput": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "electronCount": {
            "type": "number"
          },
          "coreShape": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "cell",
          "electronCount",
          "coreShape"
        ],
        "additionalProperties": false,
        "title": "ElectronDcInput"
      },
      "ElectronDcOutput": {
        "$ref": "#/$defs/Field",
        "title": "ElectronDcOutput"
      },
      "NuclearDcInput": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "signedScatteringSumFm": {
            "type": "number"
          },
          "coreShape": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "cell",
          "signedScatteringSumFm",
          "coreShape"
        ],
        "additionalProperties": false,
        "title": "NuclearDcInput"
      },
      "NuclearDcOutput": {
        "$ref": "#/$defs/Field",
        "title": "NuclearDcOutput"
      },
      "ModelPattersonCoefficientsInput": {
        "type": "object",
        "properties": {
          "reflections": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Reflection"
            }
          }
        },
        "required": [
          "reflections"
        ],
        "additionalProperties": false,
        "title": "ModelPattersonCoefficientsInput"
      },
      "ModelPattersonCoefficientsOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/PattersonCoefficient"
        },
        "title": "ModelPattersonCoefficientsOutput"
      },
      "VolumePattersonInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          }
        },
        "required": [
          "field"
        ],
        "additionalProperties": false,
        "title": "VolumePattersonInput"
      },
      "VolumePattersonOutput": {
        "$ref": "#/$defs/Field",
        "title": "VolumePattersonOutput"
      },
      "NormalizeGridLayoutInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          },
          "targetAxisOrder": {
            "enum": [
              "x-fastest",
              "z-fastest"
            ]
          },
          "targetClosure": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 3,
            "maxItems": 3
          }
        },
        "required": [
          "field",
          "targetAxisOrder",
          "targetClosure"
        ],
        "additionalProperties": false,
        "title": "NormalizeGridLayoutInput"
      },
      "NormalizeGridLayoutOutput": {
        "$ref": "#/$defs/Field",
        "title": "NormalizeGridLayoutOutput"
      },
      "IntersectHalfspacesInput": {
        "type": "object",
        "properties": {
          "normalsCartesian": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 4
          },
          "offsetsA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 4
          }
        },
        "required": [
          "normalsCartesian",
          "offsetsA"
        ],
        "additionalProperties": false,
        "title": "IntersectHalfspacesInput"
      },
      "IntersectHalfspacesOutput": {
        "$ref": "#/$defs/Mesh",
        "title": "IntersectHalfspacesOutput"
      },
      "SelectIsoEdgesInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          },
          "level": {
            "type": "number"
          }
        },
        "required": [
          "field",
          "level"
        ],
        "additionalProperties": false,
        "title": "SelectIsoEdgesInput"
      },
      "SelectIsoEdgesOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Vec3"
        },
        "title": "SelectIsoEdgesOutput"
      },
      "BuildAtomInstancesInput": {
        "type": "object",
        "properties": {
          "points": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ImagePoint"
            }
          },
          "radiusA": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "colorRGBA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": 0,
              "maximum": 1
            },
            "minItems": 4,
            "maxItems": 4
          }
        },
        "required": [
          "points",
          "radiusA",
          "colorRGBA"
        ],
        "additionalProperties": false,
        "title": "BuildAtomInstancesInput"
      },
      "BuildAtomInstancesOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/AtomInstance"
        },
        "title": "BuildAtomInstancesOutput"
      },
      "SliceTrilinearInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          },
          "originA": {
            "$ref": "#/$defs/Vec3"
          },
          "uA": {
            "$ref": "#/$defs/Vec3"
          },
          "vA": {
            "$ref": "#/$defs/Vec3"
          },
          "width": {
            "type": "integer",
            "minimum": 1
          },
          "height": {
            "type": "integer",
            "minimum": 1
          }
        },
        "required": [
          "field",
          "originA",
          "uA",
          "vA",
          "width",
          "height"
        ],
        "additionalProperties": false,
        "title": "SliceTrilinearInput"
      },
      "SliceTrilinearOutput": {
        "$ref": "#/$defs/Slice",
        "title": "SliceTrilinearOutput"
      },
      "SampleLineInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          },
          "startFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "endFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "count": {
            "type": "integer",
            "minimum": 2
          }
        },
        "required": [
          "field",
          "startFractional",
          "endFractional",
          "count"
        ],
        "additionalProperties": false,
        "title": "SampleLineInput"
      },
      "SampleLineOutput": {
        "$ref": "#/$defs/Line",
        "title": "SampleLineOutput"
      },
      "FindLocalExtremaInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          },
          "kind": {
            "enum": [
              "maximum",
              "minimum"
            ]
          },
          "limit": {
            "type": "integer",
            "minimum": 1
          }
        },
        "required": [
          "field",
          "kind",
          "limit"
        ],
        "additionalProperties": false,
        "title": "FindLocalExtremaInput"
      },
      "FindLocalExtremaOutput": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Peak"
        },
        "title": "FindLocalExtremaOutput"
      },
      "QueryPolyIntegralCapabilityInput": {
        "type": "object",
        "properties": {
          "referenceVersion": {
            "const": "3.90.9a-macos-arm64"
          }
        },
        "required": [
          "referenceVersion"
        ],
        "additionalProperties": false,
        "title": "QueryPolyIntegralCapabilityInput"
      },
      "QueryPolyIntegralCapabilityOutput": {
        "$ref": "#/$defs/Capability",
        "title": "QueryPolyIntegralCapabilityOutput"
      },
      "ConvertDensityInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/Field"
          },
          "mode": {
            "enum": [
              "lnAbs",
              "legacyLaplacian26",
              "legacyG",
              "legacyV",
              "legacyHe"
            ]
          },
          "lengthUnit": {
            "enum": [
              "angstrom",
              "bohr"
            ]
          }
        },
        "required": [
          "field",
          "mode",
          "lengthUnit"
        ],
        "additionalProperties": false,
        "title": "ConvertDensityInput"
      },
      "ConvertDensityOutput": {
        "$ref": "#/$defs/Field",
        "title": "ConvertDensityOutput"
      },
      "PlanEncodeInput": {
        "type": "object",
        "properties": {
          "profileId": {
            "type": "string"
          },
          "source": {
            "$ref": "#/$defs/EncodeSource"
          },
          "purpose": {
            "enum": [
              "save",
              "saveAs",
              "export"
            ]
          }
        },
        "required": [
          "profileId",
          "source",
          "purpose"
        ],
        "additionalProperties": false,
        "title": "PlanEncodeInput"
      },
      "PlanEncodeOutput": {
        "$ref": "#/$defs/EncodePlan",
        "title": "PlanEncodeOutput"
      },
      "Context": {
        "type": "object",
        "properties": {
          "document": {
            "$ref": "#/$defs/Id"
          },
          "epoch": {
            "$ref": "#/$defs/Id"
          },
          "baseRevision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 1,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 1,
            "maximum": 9007199254740991
          },
          "workUnits": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          }
        },
        "required": [
          "document",
          "epoch",
          "baseRevision",
          "maxResidentBytes",
          "maxOutputBytes",
          "workUnits"
        ],
        "additionalProperties": false,
        "title": "Context"
      },
      "Request": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ResolvePlacements"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ResolvePlacementsInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ExpandImages"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ExpandImagesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ChangeBasis"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ChangeBasisInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "TransportAdp"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/TransportAdpInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "FindDuplicates"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/FindDuplicatesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "SelectBondPairs"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/SelectBondPairsInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "MeasurePolyhedron"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/MeasurePolyhedronInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "TransformVector"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/TransformVectorInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "MillerPlane"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/MillerPlaneInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ClipExplicitImages"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ClipExplicitImagesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ComposeCamera"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ComposeCameraInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "MeasureImages"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/MeasureImagesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ProposeTidyResult"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ProposeTidyResultInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "PrepareMadelInput"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/PrepareMadelInputInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ConvolvePowderLines"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ConvolvePowderLinesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "SumStructureFactors"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/SumStructureFactorsInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "SynthesizeFourier"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/SynthesizeFourierInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ElectronDc"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ElectronDcInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "NuclearDc"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/NuclearDcInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ModelPattersonCoefficients"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ModelPattersonCoefficientsInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "VolumePatterson"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/VolumePattersonInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "NormalizeGridLayout"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/NormalizeGridLayoutInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "IntersectHalfspaces"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/IntersectHalfspacesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "SelectIsoEdges"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/SelectIsoEdgesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "BuildAtomInstances"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/BuildAtomInstancesInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "SliceTrilinear"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/SliceTrilinearInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "SampleLine"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/SampleLineInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "FindLocalExtrema"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/FindLocalExtremaInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "QueryPolyIntegralCapability"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/QueryPolyIntegralCapabilityInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "ConvertDensity"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/ConvertDensityInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "PlanEncode"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/PlanEncodeInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "protocol": {
                "const": "vestacraft.scientific.bounded/3"
              },
              "operation": {
                "const": "EncodeApproved"
              },
              "context": {
                "$ref": "#/$defs/Context"
              },
              "input": {
                "$ref": "#/$defs/EncodeApprovedInput"
              }
            },
            "required": [
              "protocol",
              "operation",
              "context",
              "input"
            ],
            "additionalProperties": false
          }
        ],
        "title": "Request"
      },
      "ResolvedPlacement": {
        "type": "object",
        "properties": {
          "phase": {
            "$ref": "#/$defs/Id"
          },
          "rotation": {
            "$ref": "#/$defs/Mat3"
          },
          "translationA": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "phase",
          "rotation",
          "translationA"
        ],
        "additionalProperties": false,
        "title": "ResolvedPlacement"
      },
      "MeasurementPoint": {
        "type": "object",
        "properties": {
          "pointId": {
            "$ref": "#/$defs/Id"
          },
          "cartesianA": {
            "$ref": "#/$defs/Vec3"
          }
        },
        "required": [
          "pointId",
          "cartesianA"
        ],
        "additionalProperties": false,
        "title": "MeasurementPoint"
      },
      "CartesianPointParameter": {
        "type": "object",
        "properties": {
          "pointId": {
            "$ref": "#/$defs/Id"
          },
          "axis": {
            "enum": [
              "x",
              "y",
              "z"
            ]
          },
          "unit": {
            "const": "A"
          }
        },
        "required": [
          "pointId",
          "axis",
          "unit"
        ],
        "additionalProperties": false,
        "title": "CartesianPointParameter"
      },
      "CartesianPointCovariance": {
        "type": "object",
        "properties": {
          "parameters": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/CartesianPointParameter"
            }
          },
          "matrixA2": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number"
              }
            }
          },
          "source": {
            "enum": [
              "supplied-full",
              "supplied-diagonal"
            ]
          },
          "sourceDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "parameters",
          "matrixA2",
          "source",
          "sourceDigest"
        ],
        "additionalProperties": false,
        "title": "CartesianPointCovariance"
      },
      "RawBlockForEncode": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/Resource"
          },
          "byteStart": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "byteLength": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "formatRef": {
            "type": "string"
          },
          "dependentFields": {
            "type": "array",
            "items": {
              "type": "string"
            }
          },
          "invalidated": {
            "type": "boolean"
          }
        },
        "required": [
          "source",
          "byteStart",
          "byteLength",
          "formatRef",
          "dependentFields",
          "invalidated"
        ],
        "additionalProperties": false,
        "title": "RawBlockForEncode"
      },
      "EncodeIdentity": {
        "type": "object",
        "properties": {
          "documentId": {
            "$ref": "#/$defs/Id"
          },
          "epoch": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          },
          "stateDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "documentId",
          "epoch",
          "revision",
          "stateDigest"
        ],
        "additionalProperties": false,
        "title": "EncodeIdentity"
      },
      "EncodeInventory": {
        "type": "object",
        "properties": {
          "structures": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "phaseId": {
                  "$ref": "#/$defs/Id"
                },
                "structure": {
                  "$ref": "#/$defs/Structure"
                }
              },
              "required": [
                "phaseId",
                "structure"
              ],
              "additionalProperties": false
            }
          },
          "fields": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FieldHandle"
            }
          },
          "placements": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Placement"
            }
          },
          "scene": {
            "oneOf": [
              {
                "$ref": "#/$defs/Resource"
              },
              {
                "type": "null"
              }
            ]
          },
          "comment": {
            "type": "string"
          },
          "opaqueBlocks": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/RawBlockForEncode"
            }
          }
        },
        "required": [
          "structures",
          "fields",
          "placements",
          "scene",
          "comment",
          "opaqueBlocks"
        ],
        "additionalProperties": false,
        "title": "EncodeInventory"
      },
      "EncodeSource": {
        "type": "object",
        "properties": {
          "identity": {
            "$ref": "#/$defs/EncodeIdentity"
          },
          "inventory": {
            "$ref": "#/$defs/EncodeInventory"
          }
        },
        "required": [
          "identity",
          "inventory"
        ],
        "additionalProperties": false,
        "title": "EncodeSource"
      },
      "LossField": {
        "type": "object",
        "properties": {
          "objectId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "fieldPath": {
            "type": "string"
          },
          "reason": {
            "enum": [
              "unsupported-field",
              "unsupported-block",
              "invalidated-opaque",
              "unit-convention",
              "missing-resource"
            ]
          },
          "objectKind": {
            "enum": [
              "project",
              "phase",
              "site",
              "scene",
              "field"
            ]
          }
        },
        "required": [
          "objectId",
          "fieldPath",
          "reason",
          "objectKind"
        ],
        "additionalProperties": false,
        "title": "LossField"
      },
      "LossReport1": {
        "type": "object",
        "properties": {
          "sourceDigest": {
            "$ref": "#/$defs/Digest"
          },
          "profileId": {
            "type": "string"
          },
          "droppedFields": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/LossField"
            }
          },
          "unsupportedBlocks": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/RawBlockForEncode"
            }
          },
          "requiredResources": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Digest"
            }
          },
          "purpose": {
            "enum": [
              "save",
              "saveAs",
              "export"
            ]
          }
        },
        "required": [
          "sourceDigest",
          "profileId",
          "droppedFields",
          "unsupportedBlocks",
          "requiredResources",
          "purpose"
        ],
        "additionalProperties": false,
        "title": "LossReport1"
      },
      "LossConsent1": {
        "type": "object",
        "properties": {
          "sourceDigest": {
            "$ref": "#/$defs/Digest"
          },
          "profileId": {
            "type": "string"
          },
          "lossReportSha256": {
            "$ref": "#/$defs/Digest"
          },
          "explicit": {
            "const": true
          }
        },
        "required": [
          "sourceDigest",
          "profileId",
          "lossReportSha256",
          "explicit"
        ],
        "additionalProperties": false,
        "title": "LossConsent1"
      },
      "EncodePlan": {
        "type": "object",
        "properties": {
          "sourceDigest": {
            "$ref": "#/$defs/Digest"
          },
          "profileId": {
            "type": "string"
          },
          "lossReport": {
            "$ref": "#/$defs/LossReport1"
          },
          "lossReportSha256": {
            "$ref": "#/$defs/Digest"
          },
          "requiresConsent": {
            "type": "boolean"
          },
          "purpose": {
            "enum": [
              "save",
              "saveAs",
              "export"
            ]
          }
        },
        "required": [
          "sourceDigest",
          "profileId",
          "lossReport",
          "lossReportSha256",
          "requiresConsent",
          "purpose"
        ],
        "additionalProperties": false,
        "title": "EncodePlan"
      },
      "EncodeApprovedInput": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/EncodeSource"
          },
          "plan": {
            "$ref": "#/$defs/EncodePlan"
          },
          "lossConsent": {
            "oneOf": [
              {
                "$ref": "#/$defs/LossConsent1"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "source",
          "plan",
          "lossConsent"
        ],
        "additionalProperties": false,
        "title": "EncodeApprovedInput"
      },
      "EncodeApprovedOutput": {
        "$ref": "#/$defs/Bundle",
        "title": "EncodeApprovedOutput"
      },
      "FieldHandle": {
        "type": "object",
        "properties": {
          "fieldId": {
            "$ref": "#/$defs/Id"
          },
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "field": {
            "$ref": "#/$defs/Field"
          }
        },
        "required": [
          "fieldId",
          "phaseId",
          "field"
        ],
        "additionalProperties": false,
        "title": "FieldHandle"
      },
      "SGSourceSpan": {
        "type": "object",
        "properties": {
          "resource": {
            "$ref": "#/$defs/Digest"
          },
          "byteOffset": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "byteLength": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "fieldPath": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "resource",
          "byteOffset",
          "byteLength",
          "fieldPath"
        ],
        "additionalProperties": false,
        "title": "SGSourceSpan"
      },
      "SGTableIdentity": {
        "type": "object",
        "properties": {
          "resource": {
            "$ref": "#/$defs/Digest"
          },
          "sourceId": {
            "type": "string",
            "minLength": 1
          },
          "sourceVersion": {
            "type": "string",
            "minLength": 1
          },
          "edition": {
            "type": "string",
            "minLength": 1
          },
          "licenseResource": {
            "$ref": "#/$defs/Digest"
          },
          "tableKey": {
            "type": "string",
            "minLength": 1
          },
          "tableContentDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "resource",
          "sourceId",
          "sourceVersion",
          "edition",
          "licenseResource",
          "tableKey",
          "tableContentDigest"
        ],
        "additionalProperties": false,
        "title": "SGTableIdentity"
      },
      "SGRational": {
        "type": "object",
        "properties": {
          "numerator": {
            "type": "integer",
            "minimum": -9007199254740991,
            "maximum": 9007199254740991
          },
          "denominator": {
            "type": "integer",
            "minimum": 1,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "numerator",
          "denominator"
        ],
        "additionalProperties": false,
        "title": "SGRational"
      },
      "SGSymmetryOperator3": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "fractionalMatrix": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/SGRational"
              },
              "minItems": 3,
              "maxItems": 3
            },
            "minItems": 3,
            "maxItems": 3
          },
          "fractionalTranslation": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGRational"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "timeSign": {
            "enum": [
              -1,
              1
            ]
          }
        },
        "required": [
          "id",
          "fractionalMatrix",
          "fractionalTranslation",
          "timeSign"
        ],
        "additionalProperties": false,
        "title": "SGSymmetryOperator3"
      },
      "SGGroupSetting3": {
        "type": "object",
        "properties": {
          "kind": {
            "enum": [
              "ordinary",
              "magnetic"
            ]
          },
          "symbol": {
            "type": "string",
            "minLength": 1
          },
          "hallSymbol": {
            "oneOf": [
              {
                "type": "string",
                "minLength": 1
              },
              {
                "type": "null"
              }
            ]
          },
          "bnsNumber": {
            "oneOf": [
              {
                "type": "string",
                "minLength": 1
              },
              {
                "type": "null"
              }
            ]
          },
          "ogNumber": {
            "oneOf": [
              {
                "type": "string",
                "minLength": 1
              },
              {
                "type": "null"
              }
            ]
          },
          "settingName": {
            "type": "string",
            "minLength": 1
          },
          "originChoice": {
            "type": "string",
            "minLength": 1
          },
          "changeOfBasis": {
            "$ref": "#/$defs/Mat3"
          },
          "originFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "table": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGTableIdentity"
              },
              {
                "type": "null"
              }
            ]
          },
          "operators": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSymmetryOperator3"
            }
          },
          "sourceSpans": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSourceSpan"
            }
          }
        },
        "required": [
          "kind",
          "symbol",
          "hallSymbol",
          "bnsNumber",
          "ogNumber",
          "settingName",
          "originChoice",
          "changeOfBasis",
          "originFractional",
          "table",
          "operators",
          "sourceSpans"
        ],
        "additionalProperties": false,
        "title": "SGGroupSetting3"
      },
      "SGAdp": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "iso-U"
              },
              "uA2": {
                "type": "number"
              }
            },
            "required": [
              "kind",
              "uA2"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "iso-B"
              },
              "bA2": {
                "type": "number"
              }
            },
            "required": [
              "kind",
              "bA2"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "cif-U"
              },
              "sixA2": {
                "type": "array",
                "items": {
                  "type": "number"
                },
                "minItems": 6,
                "maxItems": 6
              }
            },
            "required": [
              "kind",
              "sixA2"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "beta"
              },
              "six": {
                "type": "array",
                "items": {
                  "type": "number"
                },
                "minItems": 6,
                "maxItems": 6
              }
            },
            "required": [
              "kind",
              "six"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "cartesian-U"
              },
              "sixA2": {
                "type": "array",
                "items": {
                  "type": "number"
                },
                "minItems": 6,
                "maxItems": 6
              }
            },
            "required": [
              "kind",
              "sixA2"
            ],
            "additionalProperties": false
          }
        ],
        "title": "SGAdp"
      },
      "SGParameter": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "site-fractional"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "siteId": {
                "$ref": "#/$defs/Id"
              },
              "axis": {
                "enum": [
                  0,
                  1,
                  2
                ]
              }
            },
            "required": [
              "kind",
              "phaseId",
              "siteId",
              "axis"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "site-cartesian"
              },
              "pointId": {
                "$ref": "#/$defs/Id"
              },
              "axis": {
                "enum": [
                  0,
                  1,
                  2
                ]
              }
            },
            "required": [
              "kind",
              "pointId",
              "axis"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "cell-length"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "axis": {
                "enum": [
                  0,
                  1,
                  2
                ]
              }
            },
            "required": [
              "kind",
              "phaseId",
              "axis"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "cell-angle-degree"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "angle": {
                "enum": [
                  "alpha",
                  "beta",
                  "gamma"
                ]
              }
            },
            "required": [
              "kind",
              "phaseId",
              "angle"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "occupancy"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "siteId": {
                "$ref": "#/$defs/Id"
              }
            },
            "required": [
              "kind",
              "phaseId",
              "siteId"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "adp-component"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "siteId": {
                "$ref": "#/$defs/Id"
              },
              "component": {
                "enum": [
                  "11",
                  "22",
                  "33",
                  "12",
                  "13",
                  "23",
                  "iso"
                ]
              }
            },
            "required": [
              "kind",
              "phaseId",
              "siteId",
              "component"
            ],
            "additionalProperties": false
          }
        ],
        "title": "SGParameter"
      },
      "SGCovariance": {
        "type": "object",
        "properties": {
          "parameters": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGParameter"
            },
            "minItems": 1
          },
          "matrix": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 1
            },
            "minItems": 1
          },
          "provenance": {
            "enum": [
              "supplied-full",
              "explicit-independent-diagonal"
            ]
          },
          "sourceSpans": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSourceSpan"
            }
          }
        },
        "required": [
          "parameters",
          "matrix",
          "provenance",
          "sourceSpans"
        ],
        "additionalProperties": false,
        "title": "SGCovariance"
      },
      "SGSite": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "label": {
            "type": "string"
          },
          "species": {
            "$ref": "#/$defs/Species"
          },
          "fractional": {
            "$ref": "#/$defs/Vec3"
          },
          "occupancy": {
            "type": "number"
          },
          "adp": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGAdp"
              },
              {
                "type": "null"
              }
            ]
          },
          "sourceSpans": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSourceSpan"
            }
          }
        },
        "required": [
          "id",
          "label",
          "species",
          "fractional",
          "occupancy",
          "adp",
          "sourceSpans"
        ],
        "additionalProperties": false,
        "title": "SGSite"
      },
      "SGPeriodic3": {
        "type": "object",
        "properties": {
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "group": {
            "$ref": "#/$defs/SGGroupSetting3"
          },
          "sites": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSite"
            }
          },
          "covariance": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGCovariance"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "cell",
          "group",
          "sites",
          "covariance"
        ],
        "additionalProperties": false,
        "title": "SGPeriodic3"
      },
      "SGPhaseNode": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "label": {
            "type": "string"
          },
          "parent": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          },
          "rotation": {
            "$ref": "#/$defs/Mat3"
          },
          "translationA": {
            "$ref": "#/$defs/Vec3"
          },
          "structure": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGPeriodic3"
              },
              {
                "type": "null"
              }
            ]
          },
          "fieldIds": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            }
          },
          "shapeId": {
            "oneOf": [
              {
                "$ref": "#/$defs/Id"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "id",
          "label",
          "parent",
          "rotation",
          "translationA",
          "structure",
          "fieldIds",
          "shapeId"
        ],
        "additionalProperties": false,
        "title": "SGPhaseNode"
      },
      "SGPhaseGraph": {
        "type": "object",
        "properties": {
          "root": {
            "$ref": "#/$defs/Id"
          },
          "phases": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGPhaseNode"
            }
          }
        },
        "required": [
          "root",
          "phases"
        ],
        "additionalProperties": false,
        "title": "SGPhaseGraph"
      },
      "SGSelectedImage": {
        "type": "object",
        "properties": {
          "key": {
            "$ref": "#/$defs/Image"
          },
          "fractional": {
            "$ref": "#/$defs/Vec3"
          },
          "localCartesianA": {
            "$ref": "#/$defs/Vec3"
          },
          "worldCartesianA": {
            "$ref": "#/$defs/Vec3"
          },
          "pointId": {
            "$ref": "#/$defs/Id"
          }
        },
        "required": [
          "key",
          "fractional",
          "localCartesianA",
          "worldCartesianA",
          "pointId"
        ],
        "additionalProperties": false,
        "title": "SGSelectedImage"
      },
      "SGVector": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "anchor": {
            "$ref": "#/$defs/Image"
          },
          "components": {
            "$ref": "#/$defs/Vec3"
          },
          "basis": {
            "enum": [
              "cartesian",
              "fractional",
              "normalized-crystal-axes"
            ]
          },
          "kind": {
            "enum": [
              "polar",
              "axial"
            ]
          },
          "unit": {
            "enum": [
              "muB",
              "A",
              "dimensionless"
            ]
          },
          "displayScale": {
            "type": "number",
            "exclusiveMinimum": 0
          }
        },
        "required": [
          "id",
          "anchor",
          "components",
          "basis",
          "kind",
          "unit",
          "displayScale"
        ],
        "additionalProperties": false,
        "title": "SGVector"
      },
      "SGDuplicateQuery": {
        "type": "object",
        "properties": {
          "images": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSelectedImage"
            }
          },
          "thresholdA": {
            "type": "number",
            "minimum": 0
          },
          "grouping": {
            "const": "connected-components-explicit-images"
          },
          "merge": {
            "const": false
          }
        },
        "required": [
          "images",
          "thresholdA",
          "grouping",
          "merge"
        ],
        "additionalProperties": false,
        "title": "SGDuplicateQuery"
      },
      "SGSiteSelector": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "site"
              },
              "siteId": {
                "$ref": "#/$defs/Id"
              }
            },
            "required": [
              "kind",
              "siteId"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "element"
              },
              "element": {
                "type": "string",
                "minLength": 1
              }
            },
            "required": [
              "kind",
              "element"
            ],
            "additionalProperties": false
          }
        ],
        "title": "SGSiteSelector"
      },
      "SGBondRule": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "fromSelector": {
            "$ref": "#/$defs/SGSiteSelector"
          },
          "toSelector": {
            "$ref": "#/$defs/SGSiteSelector"
          },
          "minimumA": {
            "type": "number",
            "minimum": 0
          },
          "maximumA": {
            "type": "number",
            "minimum": 0
          },
          "thresholdMode": {
            "enum": [
              "legacy-f32-square",
              "explicit-f64"
            ]
          },
          "extraAtoms": {
            "enum": [
              "none",
              "one-endpoint-inside",
              "recursive"
            ]
          },
          "maxDepth": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxImages": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "bondValenceTable": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGTableIdentity"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "id",
          "fromSelector",
          "toSelector",
          "minimumA",
          "maximumA",
          "thresholdMode",
          "extraAtoms",
          "maxDepth",
          "maxImages",
          "bondValenceTable"
        ],
        "additionalProperties": false,
        "title": "SGBondRule"
      },
      "SGBoundary": {
        "type": "object",
        "properties": {
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "minimumFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "maximumFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "endpoints": {
            "enum": [
              "closed",
              "half-open"
            ]
          },
          "includeBondExtensions": {
            "type": "boolean"
          },
          "ruleIds": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Id"
            }
          }
        },
        "required": [
          "phaseId",
          "minimumFractional",
          "maximumFractional",
          "endpoints",
          "includeBondExtensions",
          "ruleIds"
        ],
        "additionalProperties": false,
        "title": "SGBoundary"
      },
      "SGPolyhedron": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "center": {
            "$ref": "#/$defs/Image"
          },
          "vertices": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Image"
            }
          },
          "mesh": {
            "$ref": "#/$defs/Mesh"
          },
          "boundaryRule": {
            "$ref": "#/$defs/Id"
          },
          "valenceTable": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGTableIdentity"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "id",
          "center",
          "vertices",
          "mesh",
          "boundaryRule",
          "valenceTable"
        ],
        "additionalProperties": false,
        "title": "SGPolyhedron"
      },
      "SGPlaneSpec": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "miller"
              },
              "phaseId": {
                "$ref": "#/$defs/Id"
              },
              "hkl": {
                "$ref": "#/$defs/Int3"
              },
              "order": {
                "type": "number"
              }
            },
            "required": [
              "kind",
              "phaseId",
              "hkl",
              "order"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "three-selected-points"
              },
              "points": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/SGSelectedImage"
                },
                "minItems": 3,
                "maxItems": 3
              }
            },
            "required": [
              "kind",
              "points"
            ],
            "additionalProperties": false
          }
        ],
        "title": "SGPlaneSpec"
      },
      "SGPlane": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "localUnitNormal": {
            "$ref": "#/$defs/Vec3"
          },
          "localOffsetA": {
            "type": "number"
          },
          "worldUnitNormal": {
            "$ref": "#/$defs/Vec3"
          },
          "worldOffsetA": {
            "type": "number"
          },
          "source": {
            "$ref": "#/$defs/SGPlaneSpec"
          }
        },
        "required": [
          "id",
          "localUnitNormal",
          "localOffsetA",
          "worldUnitNormal",
          "worldOffsetA",
          "source"
        ],
        "additionalProperties": false,
        "title": "SGPlane"
      },
      "SGHalfspace": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "normal": {
            "$ref": "#/$defs/Vec3"
          },
          "offsetA": {
            "type": "number"
          }
        },
        "required": [
          "id",
          "normal",
          "offsetA"
        ],
        "additionalProperties": false,
        "title": "SGHalfspace"
      },
      "SGShape": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "halfspaces": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGHalfspace"
            }
          }
        },
        "required": [
          "id",
          "phaseId",
          "halfspaces"
        ],
        "additionalProperties": false,
        "title": "SGShape"
      },
      "SGHalfspaceCertificate": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "empty"
              },
              "nonnegativeWeights": {
                "type": "array",
                "items": {
                  "type": "number",
                  "minimum": 0
                }
              },
              "contradictionMarginA": {
                "type": "number",
                "exclusiveMinimum": 0
              }
            },
            "required": [
              "kind",
              "nonnegativeWeights",
              "contradictionMarginA"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "unbounded"
              },
              "feasiblePointA": {
                "$ref": "#/$defs/Vec3"
              },
              "recessionDirection": {
                "$ref": "#/$defs/Vec3"
              }
            },
            "required": [
              "kind",
              "feasiblePointA",
              "recessionDirection"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "lower-dimensional"
              },
              "feasiblePointA": {
                "$ref": "#/$defs/Vec3"
              },
              "affineDimension": {
                "enum": [
                  0,
                  1,
                  2
                ]
              },
              "supportingEqualities": {
                "type": "array",
                "items": {
                  "$ref": "#/$defs/Id"
                }
              }
            },
            "required": [
              "kind",
              "feasiblePointA",
              "affineDimension",
              "supportingEqualities"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "bounded-full-dimensional"
              },
              "interiorPointA": {
                "$ref": "#/$defs/Vec3"
              },
              "mesh": {
                "$ref": "#/$defs/Mesh"
              }
            },
            "required": [
              "kind",
              "interiorPointA",
              "mesh"
            ],
            "additionalProperties": false
          }
        ],
        "title": "SGHalfspaceCertificate"
      },
      "SGLineWidth": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "unit": {
                "const": "device-pixel"
              },
              "value": {
                "type": "number",
                "exclusiveMinimum": 0
              }
            },
            "required": [
              "unit",
              "value"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "unit": {
                "const": "A"
              },
              "value": {
                "type": "number",
                "exclusiveMinimum": 0
              }
            },
            "required": [
              "unit",
              "value"
            ],
            "additionalProperties": false
          }
        ],
        "title": "SGLineWidth"
      },
      "SGMaterial": {
        "type": "object",
        "properties": {
          "rgba": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": 0,
              "maximum": 1
            },
            "minItems": 4,
            "maxItems": 4
          },
          "ambient": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "diffuse": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "specular": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "shininess": {
            "type": "number",
            "minimum": 0
          }
        },
        "required": [
          "rgba",
          "ambient",
          "diffuse",
          "specular",
          "shininess"
        ],
        "additionalProperties": false,
        "title": "SGMaterial"
      },
      "SGAtomStyle": {
        "type": "object",
        "properties": {
          "mode": {
            "enum": [
              "sphere",
              "ellipsoid"
            ]
          },
          "radiusA": {
            "type": "number",
            "minimum": 0
          },
          "radiusTable": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGTableIdentity"
              },
              {
                "type": "null"
              }
            ]
          },
          "material": {
            "$ref": "#/$defs/SGMaterial"
          },
          "probabilityPercent": {
            "type": "integer",
            "minimum": 1,
            "maximum": 99
          },
          "principalEllipse": {
            "type": "boolean"
          },
          "outlineWidth": {
            "$ref": "#/$defs/SGLineWidth"
          },
          "slices": {
            "type": "integer",
            "minimum": 3
          },
          "stacks": {
            "type": "integer",
            "minimum": 2
          }
        },
        "required": [
          "mode",
          "radiusA",
          "radiusTable",
          "material",
          "probabilityPercent",
          "principalEllipse",
          "outlineWidth",
          "slices",
          "stacks"
        ],
        "additionalProperties": false,
        "title": "SGAtomStyle"
      },
      "SGCamera": {
        "type": "object",
        "properties": {
          "rotation": {
            "$ref": "#/$defs/Mat3"
          },
          "panWorldA": {
            "$ref": "#/$defs/Vec3"
          },
          "zoom": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "projection": {
            "enum": [
              "parallel",
              "perspective"
            ]
          },
          "widthA": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "perspectiveVerticalFovDeg": {
            "type": "number",
            "exclusiveMinimum": 0,
            "exclusiveMaximum": 180
          },
          "nearA": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "farA": {
            "type": "number",
            "exclusiveMinimum": 0
          }
        },
        "required": [
          "rotation",
          "panWorldA",
          "zoom",
          "projection",
          "widthA",
          "perspectiveVerticalFovDeg",
          "nearA",
          "farA"
        ],
        "additionalProperties": false,
        "title": "SGCamera"
      },
      "SGNativeCameraFields": {
        "type": "object",
        "properties": {
          "sceneMatrix4": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "number"
              },
              "minItems": 4,
              "maxItems": 4
            },
            "minItems": 4,
            "maxItems": 4
          },
          "sceneShift": {
            "$ref": "#/$defs/Vec3"
          },
          "zoom": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "dispf": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "projt": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "sourceSpan": {
            "$ref": "#/$defs/SGSourceSpan"
          }
        },
        "required": [
          "sceneMatrix4",
          "sceneShift",
          "zoom",
          "dispf",
          "projt",
          "sourceSpan"
        ],
        "additionalProperties": false,
        "title": "SGNativeCameraFields"
      },
      "SGDisplay": {
        "type": "object",
        "properties": {
          "showModels": {
            "type": "boolean"
          },
          "asymmetricOnly": {
            "type": "boolean"
          },
          "dotSurface": {
            "type": "boolean"
          },
          "style": {
            "enum": [
              "ball-and-stick",
              "space-filling",
              "polyhedral",
              "wireframe",
              "stick"
            ]
          },
          "hiddenImages": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Image"
            }
          },
          "atomStyle": {
            "$ref": "#/$defs/SGAtomStyle"
          }
        },
        "required": [
          "showModels",
          "asymmetricOnly",
          "dotSurface",
          "style",
          "hiddenImages",
          "atomStyle"
        ],
        "additionalProperties": false,
        "title": "SGDisplay"
      },
      "SGMeasurement": {
        "type": "object",
        "properties": {
          "id": {
            "$ref": "#/$defs/Id"
          },
          "kind": {
            "enum": [
              "distance",
              "angle",
              "unsigned-dihedral"
            ]
          },
          "images": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Image"
            }
          },
          "value": {
            "type": "number"
          },
          "unit": {
            "enum": [
              "A",
              "degree"
            ]
          },
          "variance": {
            "oneOf": [
              {
                "type": "number",
                "minimum": 0
              },
              {
                "type": "null"
              }
            ]
          },
          "sigma": {
            "oneOf": [
              {
                "type": "number",
                "minimum": 0
              },
              {
                "type": "null"
              }
            ]
          },
          "covariance": {
            "oneOf": [
              {
                "$ref": "#/$defs/SGCovariance"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "id",
          "kind",
          "images",
          "value",
          "unit",
          "variance",
          "sigma",
          "covariance"
        ],
        "additionalProperties": false,
        "title": "SGMeasurement"
      },
      "SGRenderContext": {
        "type": "object",
        "properties": {
          "documentId": {
            "$ref": "#/$defs/Id"
          },
          "epoch": {
            "$ref": "#/$defs/Id"
          },
          "revision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          },
          "sceneDigest": {
            "$ref": "#/$defs/Digest"
          },
          "geometryDigest": {
            "$ref": "#/$defs/Digest"
          },
          "widthPx": {
            "type": "integer",
            "minimum": 1
          },
          "heightPx": {
            "type": "integer",
            "minimum": 1
          },
          "dpr": {
            "type": "number",
            "exclusiveMinimum": 0
          }
        },
        "required": [
          "documentId",
          "epoch",
          "revision",
          "sceneDigest",
          "geometryDigest",
          "widthPx",
          "heightPx",
          "dpr"
        ],
        "additionalProperties": false,
        "title": "SGRenderContext"
      },
      "SGRenderSphere": {
        "type": "object",
        "properties": {
          "image": {
            "$ref": "#/$defs/Image"
          },
          "centerWorldA": {
            "$ref": "#/$defs/Vec3"
          },
          "radiusA": {
            "type": "number",
            "minimum": 0
          },
          "material": {
            "$ref": "#/$defs/SGMaterial"
          }
        },
        "required": [
          "image",
          "centerWorldA",
          "radiusA",
          "material"
        ],
        "additionalProperties": false,
        "title": "SGRenderSphere"
      },
      "SGRenderEllipsoid": {
        "type": "object",
        "properties": {
          "image": {
            "$ref": "#/$defs/Image"
          },
          "centerWorldA": {
            "$ref": "#/$defs/Vec3"
          },
          "axesWorld": {
            "$ref": "#/$defs/Mat3"
          },
          "semiaxesA": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": 0
            },
            "minItems": 3,
            "maxItems": 3
          },
          "probabilityPercent": {
            "type": "integer",
            "minimum": 1,
            "maximum": 99
          },
          "material": {
            "$ref": "#/$defs/SGMaterial"
          }
        },
        "required": [
          "image",
          "centerWorldA",
          "axesWorld",
          "semiaxesA",
          "probabilityPercent",
          "material"
        ],
        "additionalProperties": false,
        "title": "SGRenderEllipsoid"
      },
      "SGRenderBond": {
        "type": "object",
        "properties": {
          "ruleId": {
            "$ref": "#/$defs/Id"
          },
          "images": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Image"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "endpointsWorldA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "radiusA": {
            "type": "number",
            "minimum": 0
          },
          "material": {
            "$ref": "#/$defs/SGMaterial"
          }
        },
        "required": [
          "ruleId",
          "images",
          "endpointsWorldA",
          "radiusA",
          "material"
        ],
        "additionalProperties": false,
        "title": "SGRenderBond"
      },
      "SGRenderLabel": {
        "type": "object",
        "properties": {
          "image": {
            "oneOf": [
              {
                "$ref": "#/$defs/Image"
              },
              {
                "type": "null"
              }
            ]
          },
          "positionWorldA": {
            "$ref": "#/$defs/Vec3"
          },
          "text": {
            "type": "string",
            "minLength": 1
          },
          "font": {
            "$ref": "#/$defs/SGTableIdentity"
          },
          "sizePx": {
            "type": "number",
            "exclusiveMinimum": 0
          },
          "rgba": {
            "type": "array",
            "items": {
              "type": "number",
              "minimum": 0,
              "maximum": 1
            },
            "minItems": 4,
            "maxItems": 4
          }
        },
        "required": [
          "image",
          "positionWorldA",
          "text",
          "font",
          "sizePx",
          "rgba"
        ],
        "additionalProperties": false,
        "title": "SGRenderLabel"
      },
      "SGRenderPacket": {
        "type": "object",
        "properties": {
          "context": {
            "$ref": "#/$defs/SGRenderContext"
          },
          "spheres": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGRenderSphere"
            }
          },
          "ellipsoids": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGRenderEllipsoid"
            }
          },
          "bonds": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGRenderBond"
            }
          },
          "meshes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGRenderMesh"
            }
          },
          "labels": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGRenderLabel"
            }
          },
          "camera": {
            "$ref": "#/$defs/SGCamera"
          },
          "retainedResources": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Digest"
            }
          }
        },
        "required": [
          "context",
          "spheres",
          "ellipsoids",
          "bonds",
          "meshes",
          "labels",
          "camera",
          "retainedResources"
        ],
        "additionalProperties": false,
        "title": "SGRenderPacket"
      },
      "SGReviewBundle": {
        "type": "object",
        "properties": {
          "graph": {
            "$ref": "#/$defs/SGPhaseGraph"
          },
          "vectors": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGVector"
            }
          },
          "bondRules": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGBondRule"
            }
          },
          "boundaries": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGBoundary"
            }
          },
          "planes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGPlane"
            }
          },
          "shapes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGShape"
            }
          },
          "display": {
            "$ref": "#/$defs/SGDisplay"
          },
          "camera": {
            "$ref": "#/$defs/SGCamera"
          }
        },
        "required": [
          "graph",
          "vectors",
          "bondRules",
          "boundaries",
          "planes",
          "shapes",
          "display",
          "camera"
        ],
        "additionalProperties": false,
        "title": "SGReviewBundle"
      },
      "SGRenderMesh": {
        "type": "object",
        "properties": {
          "objectId": {
            "$ref": "#/$defs/Id"
          },
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "space": {
            "const": "world-cartesian-A"
          },
          "geometry": {
            "$ref": "#/$defs/Mesh"
          },
          "material": {
            "$ref": "#/$defs/SGMaterial"
          }
        },
        "required": [
          "objectId",
          "phaseId",
          "space",
          "geometry",
          "material"
        ],
        "additionalProperties": false,
        "title": "SGRenderMesh"
      },
      "FSContext": {
        "title": "FSContext",
        "type": "object",
        "properties": {
          "document": {
            "$ref": "#/$defs/Id"
          },
          "epoch": {
            "$ref": "#/$defs/Id"
          },
          "baseRevision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          },
          "stateDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "document",
          "epoch",
          "baseRevision",
          "stateDigest"
        ],
        "additionalProperties": false
      },
      "FSByteSpan": {
        "title": "FSByteSpan",
        "type": "object",
        "properties": {
          "resource": {
            "$ref": "#/$defs/Resource"
          },
          "offset": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "length": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "spanDigest": {
            "$ref": "#/$defs/Digest"
          },
          "sourceId": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "resource",
          "offset",
          "length",
          "spanDigest",
          "sourceId"
        ],
        "additionalProperties": false
      },
      "FSFieldBinding": {
        "title": "FSFieldBinding",
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FSContext"
          },
          "handle": {
            "$ref": "#/$defs/FieldHandle"
          },
          "descriptorDigest": {
            "$ref": "#/$defs/Digest"
          },
          "encoding": {
            "enum": [
              "ieee754-little-endian"
            ]
          }
        },
        "required": [
          "base",
          "handle",
          "descriptorDigest",
          "encoding"
        ],
        "additionalProperties": false
      },
      "FSBudgetPlan": {
        "title": "FSBudgetPlan",
        "type": "object",
        "properties": {
          "inputRetainedBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "inputDecodeBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "scratchBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "outputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "transferCopyBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "historyRetainedBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "peakResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxWorkUnits": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "inputRetainedBytes",
          "inputDecodeBytes",
          "scratchBytes",
          "outputBytes",
          "transferCopyBytes",
          "historyRetainedBytes",
          "peakResidentBytes",
          "maxResidentBytes",
          "maxOutputBytes",
          "maxWorkUnits"
        ],
        "additionalProperties": false
      },
      "FSTableUse": {
        "title": "FSTableUse",
        "type": "object",
        "properties": {
          "identity": {
            "$ref": "#/$defs/SGTableIdentity"
          },
          "species": {
            "$ref": "#/$defs/Species"
          },
          "purpose": {
            "enum": [
              "xray-f0",
              "anomalous",
              "neutron-bc",
              "isotope-mass",
              "constant"
            ]
          },
          "qualificationRecordDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "identity",
          "species",
          "purpose",
          "qualificationRecordDigest"
        ],
        "additionalProperties": false
      },
      "FSProvenance": {
        "title": "FSProvenance",
        "type": "object",
        "properties": {
          "jobId": {
            "$ref": "#/$defs/Id"
          },
          "base": {
            "$ref": "#/$defs/FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "profile": {
            "enum": [
              "bounded-mathematical-design",
              "legacy-observed-selected",
              "corrected-explicit"
            ]
          },
          "algorithmVersion": {
            "type": "string",
            "minLength": 1
          },
          "parameterDigest": {
            "$ref": "#/$defs/Digest"
          },
          "inputs": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSFieldBinding"
            },
            "minItems": 0
          },
          "sourceSpans": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSByteSpan"
            },
            "minItems": 0
          },
          "tables": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSTableUse"
            },
            "minItems": 0
          }
        },
        "required": [
          "jobId",
          "base",
          "phaseId",
          "profile",
          "algorithmVersion",
          "parameterDigest",
          "inputs",
          "sourceSpans",
          "tables"
        ],
        "additionalProperties": false
      },
      "FSFieldOutput": {
        "title": "FSFieldOutput",
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/FSFieldBinding"
          },
          "provenance": {
            "$ref": "#/$defs/FSProvenance"
          },
          "budget": {
            "$ref": "#/$defs/FSBudgetPlan"
          },
          "integralCore": {
            "oneOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "integralUnit": {
            "oneOf": [
              {
                "type": "string",
                "minLength": 1
              },
              {
                "type": "null"
              }
            ]
          },
          "ownership": {
            "enum": [
              "candidate-retained",
              "engine-committed"
            ]
          }
        },
        "required": [
          "field",
          "provenance",
          "budget",
          "integralCore",
          "integralUnit",
          "ownership"
        ],
        "additionalProperties": false
      },
      "FSCoefficient": {
        "title": "FSCoefficient",
        "type": "object",
        "properties": {
          "hkl": {
            "$ref": "#/$defs/Int3"
          },
          "real": {
            "type": "number"
          },
          "imaginary": {
            "type": "number"
          }
        },
        "required": [
          "hkl",
          "real",
          "imaginary"
        ],
        "additionalProperties": false
      },
      "FSSpectrumInput": {
        "title": "FSSpectrumInput",
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "cell": {
            "$ref": "#/$defs/Cell"
          },
          "coreShape": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 4294967295
            },
            "minItems": 3,
            "maxItems": 3
          },
          "coefficients": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSCoefficient"
            },
            "minItems": 0
          },
          "dcReal": {
            "type": "number"
          },
          "amplitudeUnit": {
            "enum": [
              "electron",
              "fm",
              "electron2",
              "fm2"
            ]
          },
          "completion": {
            "enum": [
              "signed-hermitian",
              "unique-add-conjugates"
            ]
          },
          "outputDtype": {
            "enum": [
              "f32",
              "f64"
            ]
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "base",
          "phaseId",
          "cell",
          "coreShape",
          "coefficients",
          "dcReal",
          "amplitudeUnit",
          "completion",
          "outputDtype",
          "maxResidentBytes",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "FSSpectrumResult": {
        "title": "FSSpectrumResult",
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/FSFieldOutput"
          },
          "admittedCoefficients": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSCoefficient"
            },
            "minItems": 0
          },
          "dcReal": {
            "type": "number"
          },
          "normalization": {
            "enum": [
              "inverse-one-over-cell-volume"
            ]
          },
          "amplitudeUnit": {
            "enum": [
              "electron",
              "fm",
              "electron2",
              "fm2"
            ]
          }
        },
        "required": [
          "field",
          "admittedCoefficients",
          "dcReal",
          "normalization",
          "amplitudeUnit"
        ],
        "additionalProperties": false
      },
      "FSSiteAmplitude": {
        "title": "FSSiteAmplitude",
        "type": "object",
        "properties": {
          "imagePointId": {
            "$ref": "#/$defs/Id"
          },
          "hklValues": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "real": {
                  "type": "number"
                },
                "imaginary": {
                  "type": "number"
                }
              },
              "required": [
                "real",
                "imaginary"
              ],
              "additionalProperties": false
            },
            "minItems": 0
          }
        },
        "required": [
          "imagePointId",
          "hklValues"
        ],
        "additionalProperties": false
      },
      "FSSumInput": {
        "title": "FSSumInput",
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FSContext"
          },
          "phaseId": {
            "$ref": "#/$defs/Id"
          },
          "structure": {
            "$ref": "#/$defs/SGPeriodic3"
          },
          "hkl": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Int3"
            },
            "minItems": 0
          },
          "expandedImages": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/SGSelectedImage"
            },
            "minItems": 0
          },
          "siteAmplitudes": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSSiteAmplitude"
            },
            "minItems": 0
          },
          "unit": {
            "enum": [
              "electron",
              "fm"
            ]
          },
          "wavelengthA": {
            "oneOf": [
              {
                "type": "number",
                "exclusiveMinimum": 0
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "base",
          "phaseId",
          "structure",
          "hkl",
          "expandedImages",
          "siteAmplitudes",
          "unit",
          "wavelengthA"
        ],
        "additionalProperties": false
      },
      "FSReflectionRow": {
        "title": "FSReflectionRow",
        "type": "object",
        "properties": {
          "hkl": {
            "$ref": "#/$defs/Int3"
          },
          "dA": {
            "oneOf": [
              {
                "type": "number",
                "exclusiveMinimum": 0
              },
              {
                "type": "null"
              }
            ]
          },
          "real": {
            "type": "number"
          },
          "imaginary": {
            "type": "number"
          },
          "modulus": {
            "type": "number",
            "minimum": 0
          },
          "phaseDeg": {
            "oneOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "multiplicity": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "twoThetaDeg": {
            "oneOf": [
              {
                "type": "number"
              },
              {
                "type": "null"
              }
            ]
          },
          "braggStatus": {
            "enum": [
              "no-wavelength",
              "dc",
              "outside-bragg-domain",
              "valid"
            ]
          }
        },
        "required": [
          "hkl",
          "dA",
          "real",
          "imaginary",
          "modulus",
          "phaseDeg",
          "multiplicity",
          "twoThetaDeg",
          "braggStatus"
        ],
        "additionalProperties": false
      },
      "FSReflectionResult": {
        "title": "FSReflectionResult",
        "type": "object",
        "properties": {
          "provenance": {
            "$ref": "#/$defs/FSProvenance"
          },
          "rows": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSReflectionRow"
            },
            "minItems": 0
          },
          "amplitudeUnit": {
            "enum": [
              "electron",
              "fm"
            ]
          },
          "intensityNormalization": {
            "enum": [
              "not-computed"
            ]
          },
          "ordering": {
            "enum": [
              "explicit-input-order"
            ]
          }
        },
        "required": [
          "provenance",
          "rows",
          "amplitudeUnit",
          "intensityNormalization",
          "ordering"
        ],
        "additionalProperties": false
      },
      "FSLineResult": {
        "title": "FSLineResult",
        "type": "object",
        "properties": {
          "provenance": {
            "$ref": "#/$defs/FSProvenance"
          },
          "startFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "endFractional": {
            "$ref": "#/$defs/Vec3"
          },
          "count": {
            "type": "integer",
            "minimum": 2
          },
          "distanceA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 2
          },
          "values": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 2
          },
          "valueUnit": {
            "type": "string",
            "minLength": 1
          },
          "endpointPolicy": {
            "enum": [
              "include-both"
            ]
          },
          "interpolation": {
            "enum": [
              "periodic-trilinear",
              "bounded-trilinear"
            ]
          }
        },
        "required": [
          "provenance",
          "startFractional",
          "endFractional",
          "count",
          "distanceA",
          "values",
          "valueUnit",
          "endpointPolicy",
          "interpolation"
        ],
        "additionalProperties": false
      },
      "FSSliceResult": {
        "title": "FSSliceResult",
        "type": "object",
        "properties": {
          "provenance": {
            "$ref": "#/$defs/FSProvenance"
          },
          "width": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "height": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "originA": {
            "$ref": "#/$defs/Vec3"
          },
          "uStepA": {
            "$ref": "#/$defs/Vec3"
          },
          "vStepA": {
            "$ref": "#/$defs/Vec3"
          },
          "values": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 0
          },
          "unit": {
            "type": "string",
            "minLength": 1
          },
          "axisOrder": {
            "enum": [
              "u-fastest"
            ]
          },
          "endpointPolicy": {
            "enum": [
              "explicit-counts"
            ]
          }
        },
        "required": [
          "provenance",
          "width",
          "height",
          "originA",
          "uStepA",
          "vStepA",
          "values",
          "unit",
          "axisOrder",
          "endpointPolicy"
        ],
        "additionalProperties": false
      },
      "FSProjectionInput": {
        "title": "FSProjectionInput",
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/FSFieldBinding"
          },
          "width": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "height": {
            "type": "integer",
            "minimum": 1,
            "maximum": 4294967295
          },
          "originA": {
            "$ref": "#/$defs/Vec3"
          },
          "uStepA": {
            "$ref": "#/$defs/Vec3"
          },
          "vStepA": {
            "$ref": "#/$defs/Vec3"
          },
          "normalUnitCartesian": {
            "$ref": "#/$defs/Vec3"
          },
          "offsetSamplesA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 1
          },
          "weightsA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 1
          },
          "weightPolicy": {
            "enum": [
              "provided-signed-length-weights"
            ]
          },
          "profile": {
            "enum": [
              "bounded-mathematical-design",
              "legacy001-observed-selected",
              "corrected-explicit"
            ]
          }
        },
        "required": [
          "field",
          "width",
          "height",
          "originA",
          "uStepA",
          "vStepA",
          "normalUnitCartesian",
          "offsetSamplesA",
          "weightsA",
          "weightPolicy",
          "profile"
        ],
        "additionalProperties": false
      },
      "FSProjectionResult": {
        "title": "FSProjectionResult",
        "type": "object",
        "properties": {
          "slice": {
            "$ref": "#/$defs/FSSliceResult"
          },
          "sourceValueUnit": {
            "type": "string",
            "minLength": 1
          },
          "weightUnit": {
            "enum": [
              "angstrom"
            ]
          },
          "normalUnitCartesian": {
            "$ref": "#/$defs/Vec3"
          },
          "offsetSamplesA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 1
          },
          "weightsA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 1
          },
          "meaning": {
            "enum": [
              "provided-weight-quadrature"
            ]
          }
        },
        "required": [
          "slice",
          "sourceValueUnit",
          "weightUnit",
          "normalUnitCartesian",
          "offsetSamplesA",
          "weightsA",
          "meaning"
        ],
        "additionalProperties": false
      },
      "FSPeak": {
        "title": "FSPeak",
        "type": "object",
        "properties": {
          "peakId": {
            "$ref": "#/$defs/Id"
          },
          "coreIndex": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 0,
              "maximum": 9007199254740991
            },
            "minItems": 3,
            "maxItems": 3
          },
          "fractional": {
            "$ref": "#/$defs/Vec3"
          },
          "cartesianA": {
            "$ref": "#/$defs/Vec3"
          },
          "value": {
            "type": "number"
          },
          "sign": {
            "enum": [
              "positive",
              "negative",
              "zero"
            ]
          }
        },
        "required": [
          "peakId",
          "coreIndex",
          "fractional",
          "cartesianA",
          "value",
          "sign"
        ],
        "additionalProperties": false
      },
      "FSBasinAllocation": {
        "title": "FSBasinAllocation",
        "type": "object",
        "properties": {
          "peakId": {
            "$ref": "#/$defs/Id"
          },
          "integral": {
            "type": "number"
          },
          "integralUnit": {
            "type": "string",
            "minLength": 1
          },
          "volumeA3": {
            "type": "number",
            "minimum": 0
          }
        },
        "required": [
          "peakId",
          "integral",
          "integralUnit",
          "volumeA3"
        ],
        "additionalProperties": false
      },
      "FSPeakResult": {
        "title": "FSPeakResult",
        "type": "object",
        "properties": {
          "provenance": {
            "$ref": "#/$defs/FSProvenance"
          },
          "detectedCount": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "selected": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSPeak"
            },
            "minItems": 0
          },
          "selectionLimit": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "limitMeaning": {
            "enum": [
              "zero-all-selected-set-affects-basins"
            ]
          },
          "boundaryNodeCount": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "coreNodeCount": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "basins": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSBasinAllocation"
            },
            "minItems": 0
          },
          "wholeSignedIntegral": {
            "type": "number"
          },
          "wholePositiveIntegral": {
            "type": "number"
          },
          "wholeNegativeIntegral": {
            "type": "number"
          },
          "integralUnit": {
            "type": "string",
            "minLength": 1
          },
          "policy": {
            "enum": [
              "bounded-strict-extrema-only",
              "legacy-selected-voronoi26"
            ]
          }
        },
        "required": [
          "provenance",
          "detectedCount",
          "selected",
          "selectionLimit",
          "limitMeaning",
          "boundaryNodeCount",
          "coreNodeCount",
          "basins",
          "wholeSignedIntegral",
          "wholePositiveIntegral",
          "wholeNegativeIntegral",
          "integralUnit",
          "policy"
        ],
        "additionalProperties": false
      },
      "FSNonfiniteDiagnostic": {
        "title": "FSNonfiniteDiagnostic",
        "type": "object",
        "properties": {
          "base": {
            "$ref": "#/$defs/FSContext"
          },
          "input": {
            "$ref": "#/$defs/FSFieldBinding"
          },
          "mode": {
            "enum": [
              "lnAbs"
            ]
          },
          "coreIndices": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": 0,
                "maximum": 9007199254740991
              },
              "minItems": 3,
              "maxItems": 3
            },
            "minItems": 1
          },
          "special": {
            "enum": [
              "-Infinity"
            ]
          },
          "error": {
            "enum": [
              "E_DOMAIN"
            ]
          },
          "referenceOutputPreserved": {
            "oneOf": [
              {
                "$ref": "#/$defs/Resource"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "required": [
          "base",
          "input",
          "mode",
          "coreIndices",
          "special",
          "error",
          "referenceOutputPreserved"
        ],
        "additionalProperties": false
      },
      "FSConversionResult": {
        "title": "FSConversionResult",
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "status": {
                "const": "finite-field"
              },
              "mode": {
                "enum": [
                  "lnAbs",
                  "legacyLaplacian26",
                  "legacyG",
                  "legacyV",
                  "legacyHe"
                ]
              },
              "rawDensityInterpretation": {
                "enum": [
                  "ePerA3",
                  "ePerBohr3"
                ]
              },
              "output": {
                "$ref": "#/$defs/FSFieldOutput"
              }
            },
            "required": [
              "status",
              "mode",
              "rawDensityInterpretation",
              "output"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "status": {
                "const": "reference-nonfinite"
              },
              "diagnostic": {
                "$ref": "#/$defs/FSNonfiniteDiagnostic"
              }
            },
            "required": [
              "status",
              "diagnostic"
            ],
            "additionalProperties": false
          }
        ]
      },
      "FSGridOperationInput": {
        "title": "FSGridOperationInput",
        "type": "object",
        "properties": {
          "field": {
            "$ref": "#/$defs/FSFieldBinding"
          },
          "operation": {
            "enum": [
              "reindex-closure-only"
            ]
          },
          "targetAxisOrder": {
            "enum": [
              "x-fastest",
              "z-fastest"
            ]
          },
          "targetClosure": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "field",
          "operation",
          "targetAxisOrder",
          "targetClosure",
          "maxResidentBytes",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "FSNoActionResult": {
        "title": "FSNoActionResult",
        "type": "object",
        "properties": {
          "referenceVersion": {
            "const": "3.90.9a-macos-arm64"
          },
          "menuId": {
            "const": 185
          },
          "handler": {
            "const": "0x10009fedc"
          },
          "outcome": {
            "const": "NoAction"
          },
          "mutates": {
            "const": false
          },
          "numericResultAvailable": {
            "const": false
          }
        },
        "required": [
          "referenceVersion",
          "menuId",
          "handler",
          "outcome",
          "mutates",
          "numericResultAvailable"
        ],
        "additionalProperties": false
      }
    },
    "$ref": "#/$defs/FSFieldBinding",
    "profile": "vestacraft.field-science.design/1",
    "revision": "FS/1",
    "wireProtocolUnchanged": "vestacraft.scientific.bounded/3",
    "scope": "Design-only additive typed catalogue. Not a command, not production capability registration. Full14groups remain partial except selected NoAction design boundary. Existing113 bounded +39SG definitions unchanged.",
    "inherited": {
      "SG": {
        "path": "output/vesta-study/20261108-113622/reviews/20261114-191308-p1-01-f1-f6-corrections/contracts/structure-geometry.contract.json",
        "bytes": 123973,
        "sha256": "51e05f021bad4c0a6497f9e32ab70e98bb76f00e3e0b90a57124b5db38d6fad0"
      },
      "bounded": {
        "path": "output/vesta-study/20261108-113622/reviews/20261114-163802-p1-design-r9-doc-state/contracts/scientific-operations.contract.json",
        "bytes": 93808,
        "sha256": "961d2ca2700ddae04359bcaa454495205b0c63d7dd180e17c0f372bf17cbcf66"
      }
    },
    "newDefinitions": [
      "FSContext",
      "FSByteSpan",
      "FSFieldBinding",
      "FSBudgetPlan",
      "FSTableUse",
      "FSProvenance",
      "FSFieldOutput",
      "FSCoefficient",
      "FSSpectrumInput",
      "FSSpectrumResult",
      "FSSiteAmplitude",
      "FSSumInput",
      "FSReflectionRow",
      "FSReflectionResult",
      "FSLineResult",
      "FSSliceResult",
      "FSProjectionInput",
      "FSProjectionResult",
      "FSPeak",
      "FSBasinAllocation",
      "FSPeakResult",
      "FSNonfiniteDiagnostic",
      "FSConversionResult",
      "FSGridOperationInput",
      "FSNoActionResult"
    ],
    "fullDomainFrozen": false,
    "productImplemented": false,
    "runtimeValidated": false,
    "requiredSemanticValidation": "semantics.json; schema is insufficient, all resource/identity/normalization predicates mandatory"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-fractional-grid-current.schema.json": {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "urn:draft:fc:native-fractional-grid-current:1",
    "type": "object",
    "properties": {
      "schemaVersion": {
        "const": "FC-native-fractional-grid-current-1"
      },
      "captureRole": {
        "const": "actual-current-native-fractional-state"
      },
      "owner": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "EntityRef"
          },
          "objectKind": {
            "const": "source-entity"
          },
          "objectId": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "kind",
          "objectKind",
          "objectId"
        ],
        "additionalProperties": false
      },
      "dataResource": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "BytesRef"
          },
          "resourceId": {
            "type": "string",
            "minLength": 1
          },
          "sha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "byteLength": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          }
        },
        "required": [
          "kind",
          "resourceId",
          "sha256",
          "byteLength"
        ],
        "additionalProperties": false
      },
      "dataEncoding": {
        "const": "ieee754-little-endian-f32"
      },
      "pointShape": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 2,
          "maximum": 2147483647
        },
        "minItems": 3,
        "maxItems": 3
      },
      "axisOrder": {
        "const": "x-fastest"
      },
      "coordinateRole": {
        "const": "native-numerical-display-only"
      },
      "coordinatePhysicalUnit": {
        "const": "source-unresolved"
      },
      "scalarRole": {
        "const": "uninterpreted-stored-scalar"
      },
      "scalarPhysicalUnit": {
        "const": "source-unresolved"
      },
      "domainTreatment": {
        "const": "finite-including-every-stored-point"
      },
      "label": {
        "type": "string"
      },
      "ruleId": {
        "const": "draft.native-fractional-current-grid"
      },
      "ruleVersion": {
        "const": "1"
      },
      "gridDivisions": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 1,
          "maximum": 2147483647
        },
        "minItems": 3,
        "maxItems": 3
      },
      "gridToDisplayRowsF64Bits": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "string",
            "pattern": "^[0-9a-f]{16}$"
          },
          "minItems": 4,
          "maxItems": 4
        },
        "minItems": 3,
        "maxItems": 3
      }
    },
    "required": [
      "schemaVersion",
      "captureRole",
      "owner",
      "dataResource",
      "dataEncoding",
      "pointShape",
      "axisOrder",
      "coordinateRole",
      "coordinatePhysicalUnit",
      "scalarRole",
      "scalarPhysicalUnit",
      "domainTreatment",
      "label",
      "ruleId",
      "ruleVersion",
      "gridDivisions",
      "gridToDisplayRowsF64Bits"
    ],
    "additionalProperties": false
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-scalar-grid-current.schema.json": {
    "type": "object",
    "properties": {
      "schemaVersion": {
        "const": "FC-native-scalar-grid-current-1"
      },
      "captureRole": {
        "const": "actual-current-native-numerical-state"
      },
      "owner": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "EntityRef"
          },
          "objectKind": {
            "const": "source-entity"
          },
          "objectId": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "kind",
          "objectKind",
          "objectId"
        ],
        "additionalProperties": false
      },
      "dataResource": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "BytesRef"
          },
          "resourceId": {
            "type": "string",
            "minLength": 1
          },
          "sha256": {
            "type": "string",
            "pattern": "^[0-9a-f]{64}$"
          },
          "byteLength": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          }
        },
        "required": [
          "kind",
          "resourceId",
          "sha256",
          "byteLength"
        ],
        "additionalProperties": false
      },
      "maskResource": {
        "oneOf": [
          {
            "type": "null"
          },
          {
            "type": "object",
            "properties": {
              "kind": {
                "const": "BytesRef"
              },
              "resourceId": {
                "type": "string",
                "minLength": 1
              },
              "sha256": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              },
              "byteLength": {
                "type": "string",
                "pattern": "^(0|[1-9][0-9]*)$"
              }
            },
            "required": [
              "kind",
              "resourceId",
              "sha256",
              "byteLength"
            ],
            "additionalProperties": false
          }
        ]
      },
      "dataEncoding": {
        "enum": [
          "ieee754-little-endian-f32",
          "ieee754-little-endian-f64"
        ]
      },
      "pointShape": {
        "type": "array",
        "items": {
          "type": "integer",
          "minimum": 2,
          "maximum": 2147483647
        },
        "minItems": 3,
        "maxItems": 3
      },
      "axisOrder": {
        "const": "x-fastest"
      },
      "originF64Bits": {
        "type": "array",
        "items": {
          "type": "string",
          "pattern": "^[0-9a-f]{16}$"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "stepColumnsF64Bits": {
        "type": "array",
        "items": {
          "type": "array",
          "items": {
            "type": "string",
            "pattern": "^[0-9a-f]{16}$"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "minItems": 3,
        "maxItems": 3
      },
      "coordinateRole": {
        "const": "native-numerical-display-only"
      },
      "coordinatePhysicalUnit": {
        "const": "source-unresolved"
      },
      "scalarRole": {
        "const": "uninterpreted-stored-scalar"
      },
      "scalarPhysicalUnit": {
        "const": "source-unresolved"
      },
      "domainTreatment": {
        "const": "finite-including-every-stored-point"
      },
      "label": {
        "type": "string"
      },
      "ruleId": {
        "const": "draft.native-current-numerical-grid"
      },
      "ruleVersion": {
        "const": "1"
      }
    },
    "required": [
      "schemaVersion",
      "captureRole",
      "owner",
      "dataResource",
      "maskResource",
      "dataEncoding",
      "pointShape",
      "axisOrder",
      "originF64Bits",
      "stepColumnsF64Bits",
      "coordinateRole",
      "coordinatePhysicalUnit",
      "scalarRole",
      "scalarPhysicalUnit",
      "domainTreatment",
      "label",
      "ruleId",
      "ruleVersion"
    ],
    "additionalProperties": false,
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "urn:draft:fc:native-scalar-grid-current:1"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-stored-isoview.schema.json": {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "urn:draft:fc:native-stored-isoview:1",
    "type": "object",
    "properties": {
      "schemaVersion": {
        "const": "FC-native-stored-isoview-1"
      },
      "target": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "EntityRef"
          },
          "objectKind": {
            "const": "source-entity"
          },
          "objectId": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "kind",
          "objectKind",
          "objectId"
        ],
        "additionalProperties": false
      },
      "levels": {
        "type": "array",
        "minItems": 1,
        "items": {
          "type": "object",
          "properties": {
            "storedValueF32Bits": {
              "type": "string",
              "pattern": "^[0-9a-f]{8}$"
            },
            "signMode": {
              "enum": [
                "positive",
                "negative",
                "both"
              ]
            }
          },
          "required": [
            "storedValueF32Bits",
            "signMode"
          ],
          "additionalProperties": false
        }
      },
      "levelMeaning": {
        "const": "stored-scalar-only"
      },
      "ruleId": {
        "const": "draft.native-stored-scalar-isoview"
      },
      "ruleVersion": {
        "const": "1"
      }
    },
    "required": [
      "schemaVersion",
      "target",
      "levels",
      "levelMeaning",
      "ruleId",
      "ruleVersion"
    ],
    "additionalProperties": false
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/fs/functional-additions.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "p1-06-fs-functional-additions.schema.json",
    "title": "P1-06 explicit mathematical functional additions, design only",
    "$defs": {
      "FSResampleAffineInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldBinding"
          },
          "targetCoreShape": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 4294967295
            },
            "minItems": 3,
            "maxItems": 3
          },
          "targetOriginA": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Vec3"
          },
          "targetStepBasisColumnsA": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Mat3"
          },
          "targetAxisOrder": {
            "enum": [
              "x-fastest",
              "z-fastest"
            ]
          },
          "targetClosure": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "boundaryPolicy": {
            "const": "wrap-source-periodic-reject-otherwise"
          },
          "dtype": {
            "const": "f64"
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "field",
          "targetCoreShape",
          "targetOriginA",
          "targetStepBasisColumnsA",
          "targetAxisOrder",
          "targetClosure",
          "boundaryPolicy",
          "dtype",
          "maxResidentBytes",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "FSFieldLinearTerm": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldBinding"
          },
          "coefficientDimensionless": {
            "type": "number"
          }
        },
        "required": [
          "field",
          "coefficientDimensionless"
        ],
        "additionalProperties": false
      },
      "FSLinearCombineFieldsInput": {
        "type": "object",
        "properties": {
          "terms": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSFieldLinearTerm"
            },
            "minItems": 1
          },
          "targetAxisOrder": {
            "enum": [
              "x-fastest",
              "z-fastest"
            ]
          },
          "targetClosure": {
            "type": "array",
            "items": {
              "type": "boolean"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "dtype": {
            "const": "f64"
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "terms",
          "targetAxisOrder",
          "targetClosure",
          "dtype",
          "maxResidentBytes",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "FSCartesianLineInput": {
        "type": "object",
        "properties": {
          "field": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldBinding"
          },
          "startA": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Vec3"
          },
          "endA": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Vec3"
          },
          "count": {
            "type": "integer",
            "minimum": 2,
            "maximum": 4294967295
          },
          "boundaryPolicy": {
            "const": "wrap-source-periodic-reject-otherwise"
          },
          "maxResidentBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          },
          "maxOutputBytes": {
            "type": "integer",
            "minimum": 0,
            "maximum": 9007199254740991
          }
        },
        "required": [
          "field",
          "startA",
          "endA",
          "count",
          "boundaryPolicy",
          "maxResidentBytes",
          "maxOutputBytes"
        ],
        "additionalProperties": false
      },
      "FSCartesianLineOutput": {
        "type": "object",
        "properties": {
          "provenance": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSProvenance"
          },
          "startA": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Vec3"
          },
          "endA": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Vec3"
          },
          "count": {
            "type": "integer",
            "minimum": 2,
            "maximum": 4294967295
          },
          "distanceA": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 2
          },
          "values": {
            "type": "array",
            "items": {
              "type": "number"
            },
            "minItems": 2
          },
          "valueUnit": {
            "type": "string",
            "minLength": 1
          },
          "endpointPolicy": {
            "const": "include-both"
          },
          "interpolation": {
            "const": "cartesian-affine-trilinear"
          }
        },
        "required": [
          "provenance",
          "startA",
          "endA",
          "count",
          "distanceA",
          "values",
          "valueUnit",
          "endpointPolicy",
          "interpolation"
        ],
        "additionalProperties": false
      },
      "FSCurveLinearTerm": {
        "type": "object",
        "properties": {
          "curve": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Curve"
          },
          "coefficientDimensionless": {
            "type": "number"
          }
        },
        "required": [
          "curve",
          "coefficientDimensionless"
        ],
        "additionalProperties": false
      },
      "FSLinearCombineCurvesInput": {
        "type": "object",
        "properties": {
          "terms": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/FSCurveLinearTerm"
            },
            "minItems": 1
          },
          "background": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Curve"
          }
        },
        "required": [
          "terms",
          "background"
        ],
        "additionalProperties": false
      },
      "FSLinearCombineCurvesOutput": {
        "type": "object",
        "properties": {
          "scaledTermCurves": {
            "type": "array",
            "items": {
              "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Curve"
            },
            "minItems": 1
          },
          "background": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Curve"
          },
          "total": {
            "$ref": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/Curve"
          }
        },
        "required": [
          "scaledTermCurves",
          "background",
          "total"
        ],
        "additionalProperties": false
      }
    },
    "x-import": {
      "path": "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json",
      "relativePath": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json",
      "sha256": "adf539aac53ddd41b46651886b00e9d5394a96c36b83f74aeb6e35984898000a"
    },
    "x-operations": [
      {
        "operation": "ResampleAffineTrilinear",
        "input": "#/$defs/FSResampleAffineInput",
        "output": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldOutput",
        "semantics": [
          "Every target core point is explicit Cartesian affine geometry; solve source step inverse then eight-corner trilinear interpolation.",
          "Wrap source periodic axes only; reject an outside nonperiodic point. Do not clamp/zero-fill or grant target periodicity.",
          "Target dtypef64; preserve qualified source physical kind/unit; exact source context/resource identity and operation provenance.",
          "Checked output/core/closure/scratch arithmetic before allocation; singular/nonfinite geometry, nonfinite values and invalid closure/periodicity fail atomically."
        ],
        "oracle": "functional-oracles.json#/resample"
      },
      {
        "operation": "LinearCombineFields",
        "input": "#/$defs/FSLinearCombineFieldsInput",
        "output": "../../../../../20261115-062917-p1-02-o1-o4-corrections/contracts/field-science.contract.json#/$defs/FSFieldOutput",
        "semantics": [
          "Require exact current-context match and compatible physical quantity/unit/origin/step/core shape/periodicity across all terms. Normalize explicit storage order/closure before calculation.",
          "In declared term order multiply each promoted f64 value by its f64 coefficient, then addf64; no reassociation/FMA is implied. Outputf64. Nativef32 source-expression grammar remains separate.",
          "Return complete new field/resource descriptor and all term provenance; preserve inputs. Reject nonfinite and invalid closure atomically."
        ],
        "oracle": "functional-oracles.json#/arithmetic"
      },
      {
        "operation": "SampleCartesianLine",
        "input": "#/$defs/FSCartesianLineInput",
        "output": "#/$defs/FSCartesianLineOutput",
        "semantics": [
          "New Cartesian endpoint input, not a reinterpretation of existing SampleLine.startFractional/endFractional.",
          "t=j/(count-1), point=startA+t*(endA-startA), value via explicit affine interpolation; distanceA=t*norm(endA-startA). Include both endpoints and exactlycount rows.",
          "values.length=distanceA.length=count; source physical units retained; count/bytes/scratch preflight and finite validation before candidate output."
        ],
        "oracle": "functional-oracles.json#/line"
      },
      {
        "operation": "LinearCombineCurves",
        "input": "#/$defs/FSLinearCombineCurvesInput",
        "output": "#/$defs/FSLinearCombineCurvesOutput",
        "semantics": [
          "All curves require identical monotone x arrays and xUnit/yUnit, finite coefficients/values and equal x/y lengths. Coefficients dimensionless.",
          "Return every scaled phase-term curve, exact supplied background and ordered sum plus background. Do not normalize by max intensity or silently resample.",
          "Explicit decomposition of supplied curves only. Radiation/species/reflection generation and absolute native/RIETAN phase normalization remain separate requests."
        ],
        "oracle": "powder-explicit-full-curve-oracle.json#/compositionProposal"
      }
    ],
    "x-common-rules": [
      "All JSON numbers and decoded fields must be finite where a finite mathematical operation is requested; JSONSchema alone is not a finite-number runtime proof.",
      "FieldBinding complete context/resource/digest/byte-order/owner validation, candidate ownership and existing job cancel/error cleanup remain mandatory.",
      "Every output array and resource is compared in full; no DC/integral-only acceptance.",
      "These strict input/output definitions are proposed in this06 artifact only. They do not mutate prior cards/protocol3, implement an API, or certify native full-domain equivalence."
    ],
    "x-designOnly": true,
    "x-productImplemented": false
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261115-092025-p1-03-completion/native-layer-current.schema.json": {
    "type": "object",
    "properties": {
      "schemaVersion": {
        "const": "FC-native-layer-current-1"
      },
      "captureRole": {
        "const": "actual-current-native-3D-layer-population"
      },
      "document": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "EntityRef"
          },
          "objectKind": {
            "const": "document"
          },
          "objectId": {
            "type": "string",
            "minLength": 1
          }
        },
        "required": [
          "kind",
          "objectKind",
          "objectId"
        ],
        "additionalProperties": false
      },
      "orderedEntries": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "owner": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "const": "EntityRef"
                    },
                    "objectKind": {
                      "const": "phase"
                    },
                    "objectId": {
                      "type": "string",
                      "minLength": 1
                    }
                  },
                  "required": [
                    "kind",
                    "objectKind",
                    "objectId"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "const": "EntityRef"
                    },
                    "objectKind": {
                      "const": "frame"
                    },
                    "objectId": {
                      "type": "string",
                      "minLength": 1
                    }
                  },
                  "required": [
                    "kind",
                    "objectKind",
                    "objectId"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "const": "EntityRef"
                    },
                    "objectKind": {
                      "const": "source-entity"
                    },
                    "objectId": {
                      "type": "string",
                      "minLength": 1
                    }
                  },
                  "required": [
                    "kind",
                    "objectKind",
                    "objectId"
                  ],
                  "additionalProperties": false
                }
              ]
            },
            "position": {
              "type": "object",
              "properties": {
                "selector": {
                  "type": "object",
                  "properties": {
                    "capturedSelector": {
                      "type": "integer",
                      "minimum": -2147483648,
                      "maximum": 2147483647
                    },
                    "reference": {
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "const": "internal-cartesian"
                            }
                          },
                          "required": [
                            "kind"
                          ],
                          "additionalProperties": false
                        },
                        {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "const": "entry"
                            },
                            "target": {
                              "oneOf": [
                                {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "const": "EntityRef"
                                    },
                                    "objectKind": {
                                      "const": "phase"
                                    },
                                    "objectId": {
                                      "type": "string",
                                      "minLength": 1
                                    }
                                  },
                                  "required": [
                                    "kind",
                                    "objectKind",
                                    "objectId"
                                  ],
                                  "additionalProperties": false
                                },
                                {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "const": "EntityRef"
                                    },
                                    "objectKind": {
                                      "const": "frame"
                                    },
                                    "objectId": {
                                      "type": "string",
                                      "minLength": 1
                                    }
                                  },
                                  "required": [
                                    "kind",
                                    "objectKind",
                                    "objectId"
                                  ],
                                  "additionalProperties": false
                                },
                                {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "const": "EntityRef"
                                    },
                                    "objectKind": {
                                      "const": "source-entity"
                                    },
                                    "objectId": {
                                      "type": "string",
                                      "minLength": 1
                                    }
                                  },
                                  "required": [
                                    "kind",
                                    "objectKind",
                                    "objectId"
                                  ],
                                  "additionalProperties": false
                                }
                              ]
                            }
                          },
                          "required": [
                            "kind",
                            "target"
                          ],
                          "additionalProperties": false
                        },
                        {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "const": "unresolved"
                            },
                            "reason": {
                              "enum": [
                                "out-of-population",
                                "ambiguous-native-instance",
                                "unqualified-source-index"
                              ]
                            }
                          },
                          "required": [
                            "kind",
                            "reason"
                          ],
                          "additionalProperties": false
                        }
                      ]
                    }
                  },
                  "required": [
                    "capturedSelector",
                    "reference"
                  ],
                  "additionalProperties": false
                },
                "currentAnchor": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                "referenceAnchor": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                }
              },
              "required": [
                "selector",
                "currentAnchor",
                "referenceAnchor"
              ],
              "additionalProperties": false
            },
            "orientation": {
              "type": "object",
              "properties": {
                "selector": {
                  "type": "object",
                  "properties": {
                    "capturedSelector": {
                      "type": "integer",
                      "minimum": -2147483648,
                      "maximum": 2147483647
                    },
                    "reference": {
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "const": "internal-cartesian"
                            }
                          },
                          "required": [
                            "kind"
                          ],
                          "additionalProperties": false
                        },
                        {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "const": "entry"
                            },
                            "target": {
                              "oneOf": [
                                {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "const": "EntityRef"
                                    },
                                    "objectKind": {
                                      "const": "phase"
                                    },
                                    "objectId": {
                                      "type": "string",
                                      "minLength": 1
                                    }
                                  },
                                  "required": [
                                    "kind",
                                    "objectKind",
                                    "objectId"
                                  ],
                                  "additionalProperties": false
                                },
                                {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "const": "EntityRef"
                                    },
                                    "objectKind": {
                                      "const": "frame"
                                    },
                                    "objectId": {
                                      "type": "string",
                                      "minLength": 1
                                    }
                                  },
                                  "required": [
                                    "kind",
                                    "objectKind",
                                    "objectId"
                                  ],
                                  "additionalProperties": false
                                },
                                {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "const": "EntityRef"
                                    },
                                    "objectKind": {
                                      "const": "source-entity"
                                    },
                                    "objectId": {
                                      "type": "string",
                                      "minLength": 1
                                    }
                                  },
                                  "required": [
                                    "kind",
                                    "objectKind",
                                    "objectId"
                                  ],
                                  "additionalProperties": false
                                }
                              ]
                            }
                          },
                          "required": [
                            "kind",
                            "target"
                          ],
                          "additionalProperties": false
                        },
                        {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "const": "unresolved"
                            },
                            "reason": {
                              "enum": [
                                "out-of-population",
                                "ambiguous-native-instance",
                                "unqualified-source-index"
                              ]
                            }
                          },
                          "required": [
                            "kind",
                            "reason"
                          ],
                          "additionalProperties": false
                        }
                      ]
                    }
                  },
                  "required": [
                    "capturedSelector",
                    "reference"
                  ],
                  "additionalProperties": false
                },
                "storedFlagBits": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{8}$"
                },
                "currentFirst": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                "referenceFirst": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                "currentSecond": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                },
                "referenceSecond": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "kind": {
                        "const": "F64Bits"
                      },
                      "ieeeHexBE": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{16}$"
                      }
                    },
                    "required": [
                      "kind",
                      "ieeeHexBE"
                    ],
                    "additionalProperties": false
                  },
                  "minItems": 3,
                  "maxItems": 3
                }
              },
              "required": [
                "selector",
                "storedFlagBits",
                "currentFirst",
                "referenceFirst",
                "currentSecond",
                "referenceSecond"
              ],
              "additionalProperties": false
            },
            "effectiveNativeMatrix4": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "kind": {
                    "const": "F64Bits"
                  },
                  "ieeeHexBE": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{16}$"
                  }
                },
                "required": [
                  "kind",
                  "ieeeHexBE"
                ],
                "additionalProperties": false
              },
              "minItems": 16,
              "maxItems": 16
            },
            "effectiveNativeTranslation": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "kind": {
                    "const": "F32Bits"
                  },
                  "ieeeHexBE": {
                    "type": "string",
                    "pattern": "^[0-9a-f]{8}$"
                  }
                },
                "required": [
                  "kind",
                  "ieeeHexBE"
                ],
                "additionalProperties": false
              },
              "minItems": 3,
              "maxItems": 3
            }
          },
          "required": [
            "owner",
            "position",
            "orientation",
            "effectiveNativeMatrix4",
            "effectiveNativeTranslation"
          ],
          "additionalProperties": false
        }
      }
    },
    "required": [
      "schemaVersion",
      "captureRole",
      "document",
      "orderedEntries"
    ],
    "additionalProperties": false,
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "urn:draft:fc:native-layer-current:1",
    "title": "Complete actual current selected3D native layer population; immutable source observations separate"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/science-supplement/native-sampling.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:design:native-sampling:2",
    "$defs": {
      "F32Bits": {
        "$ref": "../subtasks/fs/fs2-scientific-successor.schema.json#/$defs/F32Bits"
      },
      "F64Bits": {
        "$ref": "../subtasks/fs/fs2-scientific-successor.schema.json#/$defs/F64Bits"
      },
      "ScalarSource": {
        "$ref": "../subtasks/fs/fs2-scientific-successor.schema.json#/$defs/ScalarSource"
      },
      "Vec3F32Bits": {
        "$ref": "../subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Vec3F32Bits"
      },
      "Vec3F64Bits": {
        "$ref": "../subtasks/fs/fs2-scientific-successor.schema.json#/$defs/Vec3F64Bits"
      },
      "SHA256": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "NativeSamplerContext": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "ownerPhaseId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "coreDivisions": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483646
            },
            "minItems": 3,
            "maxItems": 3
          },
          "expandedScalarF32Resource": {
            "$ref": "#/$defs/SHA256"
          },
          "sourceCartesianColumnsA": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3,
            "maxItems": 3
          },
          "inputChart": {
            "const": "native-fractional-periodic3"
          },
          "producerRuleDigest": {
            "$ref": "#/$defs/SHA256"
          }
        },
        "required": [
          "source",
          "ownerPhaseId",
          "coreDivisions",
          "expandedScalarF32Resource",
          "sourceCartesianColumnsA",
          "inputChart",
          "producerRuleDigest"
        ],
        "additionalProperties": false,
        "x-authority": "Captured by Engine from exact current source/phase; client numbers/resource refs do not certify current identity, chart, layout, unit or owner. Revalidate full source binding before execution.",
        "x-invariants": [
          "exact (Nx+1)*(Ny+1)*(Nz+1)*4 bytes, x-fastest, all axis closure planes match",
          "native f32 fractional chart and captured actual C2O linear3x3; generic/affine field cannot be relabeled merely to fit this type",
          "index counts/products/f32point*N/floor andi32 operations admitted only if checked safe; original OOB/overflow maps source-failure without mutation"
        ]
      },
      "NativePointSampleInput": {
        "type": "object",
        "properties": {
          "context": {
            "$ref": "#/$defs/NativeSamplerContext"
          },
          "fractionalF32": {
            "$ref": "#/$defs/Vec3F32Bits"
          }
        },
        "required": [
          "context",
          "fractionalF32"
        ],
        "additionalProperties": false
      },
      "NativePointSampleOutput": {
        "type": "object",
        "properties": {
          "valueF32Bits": {
            "$ref": "#/$defs/F32Bits"
          }
        },
        "required": [
          "valueF32Bits"
        ],
        "additionalProperties": false
      },
      "NativeLineProfileInput": {
        "type": "object",
        "properties": {
          "context": {
            "$ref": "#/$defs/NativeSamplerContext"
          },
          "point1FractionalF32": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "point2FractionalF32": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "nativeCountI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "titleUtf8": {
            "type": "string"
          }
        },
        "required": [
          "context",
          "point1FractionalF32",
          "point2FractionalF32",
          "nativeCountI32",
          "titleUtf8"
        ],
        "additionalProperties": false
      },
      "NativeLineSample": {
        "type": "object",
        "properties": {
          "fractionalF32": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "distanceAF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "valueF32Bits": {
            "$ref": "#/$defs/F32Bits"
          }
        },
        "required": [
          "fractionalF32",
          "distanceAF32Bits",
          "valueF32Bits"
        ],
        "additionalProperties": false
      },
      "NativeLineComplete": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "complete-line-profile"
          },
          "samples": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeLineSample"
            }
          },
          "textUtf8": {
            "type": "string"
          },
          "encoding": {
            "const": "UTF-8; numeric C-locale"
          }
        },
        "required": [
          "kind",
          "samples",
          "textUtf8",
          "encoding"
        ],
        "additionalProperties": false
      },
      "NativeLineEmpty": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "reference-header-only"
          },
          "nativePrintedCountI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "samples": {
            "type": "array",
            "maxItems": 0
          },
          "textUtf8": {
            "type": "string"
          }
        },
        "required": [
          "kind",
          "nativePrintedCountI32",
          "samples",
          "textUtf8"
        ],
        "additionalProperties": false
      },
      "NativeReferenceFailure": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "reference-failure"
          },
          "code": {
            "enum": [
              "E_NATIVE_COUNT_ONE_NONFINITE",
              "E_NATIVE_INDEX_ARITHMETIC",
              "E_NATIVE_NONFINITE_SAMPLE",
              "E_NATIVE_CONTOUR_DOMAIN"
            ]
          },
          "oldModelPreserved": {
            "const": true
          },
          "noCandidatePublished": {
            "const": true
          }
        },
        "required": [
          "kind",
          "code",
          "oldModelPreserved",
          "noCandidatePublished"
        ],
        "additionalProperties": false
      },
      "NativeLineProfileOutput": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeLineComplete"
          },
          {
            "$ref": "#/$defs/NativeLineEmpty"
          },
          {
            "$ref": "#/$defs/NativeReferenceFailure"
          }
        ]
      },
      "ContourLevelMetadata6": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/F32Bits"
        },
        "minItems": 6,
        "maxItems": 6
      },
      "ContourSettingsLinear": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "native-linear"
          },
          "enabled": {
            "type": "boolean"
          },
          "minimumF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "maximumF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "stepF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "majorIntervalI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          }
        },
        "required": [
          "kind",
          "enabled",
          "minimumF32Bits",
          "maximumF32Bits",
          "stepF32Bits",
          "majorIntervalI32"
        ],
        "additionalProperties": false
      },
      "ContourSettingsGeometric": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "native-geometric"
          },
          "enabled": {
            "type": "boolean"
          },
          "minimumExponentI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "maximumExponentI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "subdivisionsI32": {
            "type": "integer",
            "minimum": -2147483648,
            "maximum": 2147483647
          },
          "baseF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "coefficientF32Bits": {
            "$ref": "#/$defs/F32Bits"
          }
        },
        "required": [
          "kind",
          "enabled",
          "minimumExponentI32",
          "maximumExponentI32",
          "subdivisionsI32",
          "baseF32Bits",
          "coefficientF32Bits"
        ],
        "additionalProperties": false
      },
      "NativeContourLevelsInput": {
        "type": "object",
        "properties": {
          "dataMinimumF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "dataMaximumF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "settings": {
            "oneOf": [
              {
                "$ref": "#/$defs/ContourSettingsLinear"
              },
              {
                "$ref": "#/$defs/ContourSettingsGeometric"
              }
            ]
          }
        },
        "required": [
          "dataMinimumF32Bits",
          "dataMaximumF32Bits",
          "settings"
        ],
        "additionalProperties": false
      },
      "NativeContourLevelsOutput": {
        "type": "object",
        "properties": {
          "metadata": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ContourLevelMetadata6"
            }
          }
        },
        "required": [
          "metadata"
        ],
        "additionalProperties": false
      },
      "NativeContourSegmentsInput": {
        "type": "object",
        "properties": {
          "vertices": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            }
          },
          "values": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            }
          },
          "polygonIndices": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": -2147483648,
              "maximum": 2147483647
            }
          },
          "levelMetadata": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/ContourLevelMetadata6"
            }
          }
        },
        "required": [
          "vertices",
          "values",
          "polygonIndices",
          "levelMetadata"
        ],
        "additionalProperties": false
      },
      "NativeContourSegment": {
        "type": "object",
        "properties": {
          "levelOrdinal0": {
            "type": "integer",
            "minimum": 0
          },
          "endpoints": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 2,
            "maxItems": 2
          }
        },
        "required": [
          "levelOrdinal0",
          "endpoints"
        ],
        "additionalProperties": false
      },
      "NativeContourSegmentsOutput": {
        "type": "object",
        "properties": {
          "segments": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/NativeContourSegment"
            }
          },
          "unpairedFinalIntersections": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "levelOrdinal0": {
                  "type": "integer",
                  "minimum": 0
                },
                "point": {
                  "$ref": "#/$defs/Vec3F32Bits"
                }
              },
              "required": [
                "levelOrdinal0",
                "point"
              ],
              "additionalProperties": false
            }
          }
        },
        "required": [
          "segments",
          "unpairedFinalIntersections"
        ],
        "additionalProperties": false
      },
      "NativePolyIntegralInput": {
        "type": "object",
        "properties": {
          "profile": {
            "const": "VESTA-3.90.9a-macos-arm64-menu185"
          },
          "documentId": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "epoch": {
            "type": "string",
            "pattern": "^[0-9a-f]{32}$"
          },
          "expectedRevision": {
            "type": "string",
            "pattern": "^(0|[1-9][0-9]*)$"
          }
        },
        "required": [
          "profile",
          "documentId",
          "epoch",
          "expectedRevision"
        ],
        "additionalProperties": false
      },
      "NativePolyIntegralOutput": {
        "type": "object",
        "properties": {
          "kind": {
            "const": "reference-no-op"
          },
          "numericIntegrationPerformed": {
            "const": false
          },
          "modelChanged": {
            "const": false
          },
          "outputAppendUtf8": {
            "const": ""
          },
          "warningCode": {
            "const": "W_REFERENCE_NOOP"
          }
        },
        "required": [
          "kind",
          "numericIntegrationPerformed",
          "modelChanged",
          "outputAppendUtf8",
          "warningCode"
        ],
        "additionalProperties": false
      }
    },
    "title": "06 typed native line, contour and selected no-op successor; no product implementation"
  },
  "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-080304-full-design-closure/subtasks/slice-geometry/slice-geometry.schema.json": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:vestacraft:design:slice-geometry:2",
    "title": "D=3 original oblique slice and thickness projection design",
    "description": "Closed scientific-boundary inputs/results; imported FS leaf definitions only, not a recursive top-level Request. All decoded FP bit patterns must be finite and pass explicit source semantic assertions. H is derived by Set_Base_Matrix, never accepted as caller free matrix.",
    "oneOf": [
      {
        "$ref": "#/$defs/NativeObliqueSliceInput"
      },
      {
        "$ref": "#/$defs/NativePeriodicSliceInput"
      }
    ],
    "$defs": {
      "ScalarSource": {
        "$ref": "../fs/fs2-scientific-successor.schema.json#/$defs/ScalarSource"
      },
      "F32Bits": {
        "$ref": "../fs/fs2-scientific-successor.schema.json#/$defs/F32Bits"
      },
      "F64Bits": {
        "$ref": "../fs/fs2-scientific-successor.schema.json#/$defs/F64Bits"
      },
      "Vec3F32Bits": {
        "$ref": "../fs/fs2-scientific-successor.schema.json#/$defs/Vec3F32Bits"
      },
      "Vec3F64Bits": {
        "$ref": "../fs/fs2-scientific-successor.schema.json#/$defs/Vec3F64Bits"
      },
      "Mat4ColumnsF64Bits": {
        "$ref": "../fs/fs2-scientific-successor.schema.json#/$defs/Mat4ColumnsF64Bits"
      },
      "Digest": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$"
      },
      "U32": {
        "type": "integer",
        "minimum": 0,
        "maximum": 4294967295
      },
      "Mat3RowsF64Bits": {
        "type": "array",
        "items": {
          "$ref": "#/$defs/Vec3F64Bits"
        },
        "minItems": 3,
        "maxItems": 3
      },
      "NativeSliceFrame": {
        "type": "object",
        "properties": {
          "dimensionD": {
            "const": 3
          },
          "crystallographicToOrthoRowsF64Bits": {
            "$ref": "#/$defs/Mat3RowsF64Bits"
          },
          "orthoToCrystallographicRowsF64Bits": {
            "$ref": "#/$defs/Mat3RowsF64Bits"
          },
          "currentC2XYZColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "sourceStateDigest": {
            "$ref": "#/$defs/Digest"
          }
        },
        "required": [
          "dimensionD",
          "crystallographicToOrthoRowsF64Bits",
          "orthoToCrystallographicRowsF64Bits",
          "currentC2XYZColumnsF64Bits",
          "sourceStateDigest"
        ],
        "additionalProperties": false
      },
      "NativeOrthoCutoffPlaneGeometry": {
        "type": "object",
        "properties": {
          "normalOrthoF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "distanceAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          }
        },
        "required": [
          "normalOrthoF64Bits",
          "distanceAF64Bits"
        ],
        "additionalProperties": false
      },
      "NativeOrthoCutoffPlane": {
        "type": "object",
        "properties": {
          "sourcePlaneDigest": {
            "$ref": "#/$defs/Digest"
          },
          "geometry": {
            "$ref": "#/$defs/NativeOrthoCutoffPlaneGeometry"
          }
        },
        "required": [
          "sourcePlaneDigest",
          "geometry"
        ],
        "additionalProperties": false
      },
      "NativeFractionalBox": {
        "type": "object",
        "properties": {
          "xMinMaxF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "yMinMaxF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "zMinMaxF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 2,
            "maxItems": 2
          }
        },
        "required": [
          "xMinMaxF32Bits",
          "yMinMaxF32Bits",
          "zMinMaxF32Bits"
        ],
        "additionalProperties": false
      },
      "NativeScalarWindow": {
        "type": "object",
        "properties": {
          "lowerF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "upperF32Bits": {
            "$ref": "#/$defs/F32Bits"
          }
        },
        "required": [
          "lowerF32Bits",
          "upperF32Bits"
        ],
        "additionalProperties": false
      },
      "NativeContourSettings": {
        "oneOf": [
          {
            "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/ContourSettingsLinear"
          },
          {
            "$ref": "../../science-supplement/native-sampling.schema.json#/$defs/ContourSettingsGeometric"
          }
        ]
      },
      "NativeObliqueSliceInput": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "frame": {
            "$ref": "#/$defs/NativeSliceFrame"
          },
          "gridIntervalsXYZ": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483646
            },
            "minItems": 3,
            "maxItems": 3
          },
          "sourceNormalF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "sourceStateDigest": {
            "$ref": "#/$defs/Digest"
          },
          "kind": {
            "const": "native-bounding-box-oblique-slice"
          },
          "distanceAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "fractionalBox": {
            "$ref": "#/$defs/NativeFractionalBox"
          },
          "scalarWindow": {
            "$ref": "#/$defs/NativeScalarWindow"
          },
          "sourceDataRangeF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "contourSettings": {
            "$ref": "#/$defs/NativeContourSettings"
          }
        },
        "required": [
          "source",
          "frame",
          "gridIntervalsXYZ",
          "sourceNormalF64Bits",
          "sourceStateDigest",
          "kind",
          "distanceAF64Bits",
          "fractionalBox",
          "scalarWindow",
          "sourceDataRangeF32Bits",
          "contourSettings"
        ],
        "additionalProperties": false
      },
      "NativeSlicePlaneRecipe": {
        "oneOf": [
          {
            "type": "object",
            "properties": {
              "mode": {
                "const": "two-vector-reciprocal-plane"
              },
              "sourceStyleByte": {
                "type": "integer",
                "minimum": 0,
                "maximum": 255
              }
            },
            "required": [
              "mode",
              "sourceStyleByte"
            ],
            "additionalProperties": false
          },
          {
            "type": "object",
            "properties": {
              "mode": {
                "const": "project-along-direct-hkl"
              },
              "sourceStyleByte": {
                "type": "integer",
                "minimum": 0,
                "maximum": 255
              },
              "upperDistanceAF64Bits": {
                "$ref": "#/$defs/F64Bits"
              }
            },
            "required": [
              "mode",
              "sourceStyleByte",
              "upperDistanceAF64Bits"
            ],
            "additionalProperties": false
          }
        ]
      },
      "NativePeriodicSliceInput": {
        "type": "object",
        "properties": {
          "source": {
            "$ref": "#/$defs/ScalarSource"
          },
          "frame": {
            "$ref": "#/$defs/NativeSliceFrame"
          },
          "gridIntervalsXYZ": {
            "type": "array",
            "items": {
              "type": "integer",
              "minimum": 1,
              "maximum": 2147483646
            },
            "minItems": 3,
            "maxItems": 3
          },
          "sourceNormalF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "sourceStateDigest": {
            "$ref": "#/$defs/Digest"
          },
          "centerFractionalF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "span1FractionalF32Bits": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "span2FractionalF32Bits": {
            "$ref": "#/$defs/Vec3F32Bits"
          },
          "lowerDistanceAF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "sourceDataRangeF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "contourSettings": {
            "$ref": "#/$defs/NativeContourSettings"
          },
          "kind": {
            "const": "native-periodic-plane-or-thickness-projection"
          },
          "planeRecipe": {
            "$ref": "#/$defs/NativeSlicePlaneRecipe"
          }
        },
        "required": [
          "source",
          "frame",
          "gridIntervalsXYZ",
          "sourceNormalF64Bits",
          "sourceStateDigest",
          "centerFractionalF64Bits",
          "span1FractionalF32Bits",
          "span2FractionalF32Bits",
          "lowerDistanceAF64Bits",
          "sourceDataRangeF32Bits",
          "contourSettings",
          "kind",
          "planeRecipe"
        ],
        "additionalProperties": false
      },
      "NativePlanePolygon": {
        "type": "object",
        "properties": {
          "verticesCurrentXYZF64Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F64Bits"
            },
            "minItems": 3
          },
          "orderedClosedCycle": {
            "const": true
          }
        },
        "required": [
          "verticesCurrentXYZF64Bits",
          "orderedClosedCycle"
        ],
        "additionalProperties": false
      },
      "NativeSliceMesh": {
        "type": "object",
        "properties": {
          "verticesPlaneF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 3
          },
          "valuesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 3
          },
          "triangles": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/U32"
              },
              "minItems": 3,
              "maxItems": 3
            },
            "minItems": 1
          }
        },
        "required": [
          "verticesPlaneF32Bits",
          "valuesF32Bits",
          "triangles"
        ],
        "additionalProperties": false
      },
      "NativeObliqueSliceComplete": {
        "type": "object",
        "properties": {
          "sourceStateDigest": {
            "$ref": "#/$defs/Digest"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          },
          "planeHColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "kind": {
            "const": "complete-oblique-slice"
          },
          "planeConstantF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "normalOrthoUnnormalizedF64Bits": {
            "$ref": "#/$defs/Vec3F64Bits"
          },
          "roundedIndexBoundsXYZ": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "type": "integer",
                "minimum": -2147483648,
                "maximum": 2147483647
              },
              "minItems": 2,
              "maxItems": 2
            },
            "minItems": 3,
            "maxItems": 3
          },
          "fullBoundary": {
            "$ref": "#/$defs/NativePlanePolygon"
          },
          "meshAfterScalarWindow": {
            "$ref": "#/$defs/NativeSliceMesh"
          },
          "contourConsumer": {
            "const": "native-contour-levels-then-segments/2"
          }
        },
        "required": [
          "sourceStateDigest",
          "sourceRuleDigest",
          "planeHColumnsF64Bits",
          "kind",
          "planeConstantF64Bits",
          "normalOrthoUnnormalizedF64Bits",
          "roundedIndexBoundsXYZ",
          "fullBoundary",
          "meshAfterScalarWindow",
          "contourConsumer"
        ],
        "additionalProperties": false
      },
      "NativeObliqueSliceEmpty": {
        "type": "object",
        "properties": {
          "sourceStateDigest": {
            "$ref": "#/$defs/Digest"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          },
          "planeHColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "kind": {
            "const": "empty-oblique-slice"
          },
          "reason": {
            "enum": [
              "outside-box",
              "degenerate-intersection",
              "scalar-window-empty"
            ]
          }
        },
        "required": [
          "sourceStateDigest",
          "sourceRuleDigest",
          "planeHColumnsF64Bits",
          "kind",
          "reason"
        ],
        "additionalProperties": false
      },
      "NativeObliqueSliceOutput": {
        "oneOf": [
          {
            "$ref": "#/$defs/NativeObliqueSliceComplete"
          },
          {
            "$ref": "#/$defs/NativeObliqueSliceEmpty"
          }
        ]
      },
      "NativePeriodicSliceOutput": {
        "type": "object",
        "properties": {
          "sourceStateDigest": {
            "$ref": "#/$defs/Digest"
          },
          "sourceRuleDigest": {
            "$ref": "#/$defs/Digest"
          },
          "planeHColumnsF64Bits": {
            "$ref": "#/$defs/Mat4ColumnsF64Bits"
          },
          "kind": {
            "const": "complete-periodic-slice-array"
          },
          "intervalsUV": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/U32"
            },
            "minItems": 2,
            "maxItems": 2
          },
          "normalIntervalsU32": {
            "$ref": "#/$defs/U32"
          },
          "continuousLayerDeltaF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "additionalLayerCountU32": {
            "$ref": "#/$defs/U32"
          },
          "thicknessWeightF32Bits": {
            "$ref": "#/$defs/F32Bits"
          },
          "lowerPlaneConstantF64Bits": {
            "$ref": "#/$defs/F64Bits"
          },
          "upperPlaneConstantF64Bits": {
            "oneOf": [
              {
                "type": "null"
              },
              {
                "$ref": "#/$defs/F64Bits"
              }
            ]
          },
          "sampleFractionalPointsF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 1
          },
          "sampleValuesPerLayerF32Bits": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/F32Bits"
              },
              "minItems": 1
            },
            "minItems": 1
          },
          "renderPositionsPlaneF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/Vec3F32Bits"
            },
            "minItems": 1
          },
          "completeValuesF32Bits": {
            "type": "array",
            "items": {
              "$ref": "#/$defs/F32Bits"
            },
            "minItems": 1
          },
          "contourQuads": {
            "type": "array",
            "items": {
              "type": "array",
              "items": {
                "$ref": "#/$defs/U32"
              },
              "minItems": 4,
              "maxItems": 4
            }
          },
          "valueQuantity": {
            "enum": [
              "source-quantity",
              "source-quantity-times-angstrom"
            ]
          },
          "contourConsumer": {
            "const": "native-contour-levels-then-segments/2"
          }
        },
        "required": [
          "sourceStateDigest",
          "sourceRuleDigest",
          "planeHColumnsF64Bits",
          "kind",
          "intervalsUV",
          "normalIntervalsU32",
          "continuousLayerDeltaF64Bits",
          "additionalLayerCountU32",
          "thicknessWeightF32Bits",
          "lowerPlaneConstantF64Bits",
          "upperPlaneConstantF64Bits",
          "sampleFractionalPointsF32Bits",
          "sampleValuesPerLayerF32Bits",
          "renderPositionsPlaneF32Bits",
          "completeValuesF32Bits",
          "contourQuads",
          "valueQuantity",
          "contourConsumer"
        ],
        "additionalProperties": false
      }
    }
  }
}
;
export const nativeFile = "/Users/agilecho/code/Draft/output/vesta-study/20261108-113622/reviews/20261118-055835-p1-06-implementation-readiness/attempts/20261118-134209-two-residuals/contracts/native-session-v2.schema.json";
export const {decodeResource,encodeResource} = codecRuntime(resourceBindings,schemaCatalog,nativeFile);
