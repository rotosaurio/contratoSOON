/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/launchpadinsoon.json`.
 */
export type Launchpadinsoon = {
  "address": "74VT9QxrMB8gWYynpS7m9bxAygWy6tTAqHELeDsVmRNV",
  "metadata": {
    "name": "launchpadinsoon",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor for the SOON network"
  },
  "instructions": [
    {
      "name": "addToWhitelist",
      "discriminator": [
        157,
        211,
        52,
        54,
        144,
        81,
        5,
        55
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "user",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "buyTokens",
      "discriminator": [
        189,
        21,
        230,
        133,
        247,
        2,
        110,
        42
      ],
      "accounts": [
        {
          "name": "presale",
          "writable": true
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "buyerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenVault",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true
        },
        {
          "name": "saleAuthority"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "globalStats",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "presaleId",
          "type": "u64"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimTokens",
      "discriminator": [
        108,
        216,
        210,
        231,
        0,
        212,
        42,
        64
      ],
      "accounts": [
        {
          "name": "presale",
          "writable": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "tokenVault",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "globalStats",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "createVesting",
      "discriminator": [
        135,
        184,
        171,
        156,
        197,
        162,
        246,
        44
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "user",
          "signer": true
        },
        {
          "name": "globalStats",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "releaseTime",
          "type": "i64"
        }
      ]
    },
    {
      "name": "getGlobalStats",
      "discriminator": [
        93,
        120,
        236,
        236,
        175,
        200,
        64,
        245
      ],
      "accounts": [
        {
          "name": "globalStats"
        }
      ],
      "args": [],
      "returns": {
        "defined": {
          "name": "globalStats"
        }
      }
    },
    {
      "name": "getPresaleStats",
      "discriminator": [
        45,
        102,
        38,
        28,
        246,
        248,
        14,
        106
      ],
      "accounts": [
        {
          "name": "presale"
        }
      ],
      "args": [],
      "returns": {
        "defined": {
          "name": "presaleStats"
        }
      }
    },
    {
      "name": "getUserStats",
      "discriminator": [
        38,
        55,
        50,
        132,
        115,
        127,
        50,
        32
      ],
      "accounts": [
        {
          "name": "presale"
        },
        {
          "name": "user",
          "signer": true
        }
      ],
      "args": [],
      "returns": {
        "defined": {
          "name": "userStats"
        }
      }
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true,
          "signer": true
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "totalTokens",
          "type": "u64"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "startTime",
          "type": "i64"
        },
        {
          "name": "endTime",
          "type": "i64"
        },
        {
          "name": "vestingEndTime",
          "type": "i64"
        },
        {
          "name": "raiseGoal",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "initializeGlobalStats",
      "discriminator": [
        57,
        82,
        52,
        126,
        182,
        236,
        5,
        131
      ],
      "accounts": [
        {
          "name": "globalStats",
          "writable": true,
          "signer": true
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializePresale",
      "discriminator": [
        9,
        174,
        12,
        126,
        150,
        119,
        68,
        100
      ],
      "accounts": [
        {
          "name": "presale",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "creatorTokenAccount",
          "writable": true
        },
        {
          "name": "tokenVault",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "commissionVault",
          "writable": true
        },
        {
          "name": "globalStats",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "id",
          "type": "u64"
        },
        {
          "name": "totalTokens",
          "type": "u64"
        },
        {
          "name": "price",
          "type": "u64"
        },
        {
          "name": "startTime",
          "type": "i64"
        },
        {
          "name": "endTime",
          "type": "i64"
        },
        {
          "name": "vestingEndTime",
          "type": "i64"
        },
        {
          "name": "raiseGoal",
          "type": "u64"
        },
        {
          "name": "bump",
          "type": "u8"
        },
        {
          "name": "maxEntries",
          "type": "u64"
        }
      ]
    },
    {
      "name": "pauseContract",
      "discriminator": [
        210,
        36,
        5,
        85,
        177,
        65,
        35,
        89
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "pauseSale",
      "discriminator": [
        120,
        107,
        163,
        108,
        19,
        201,
        121,
        223
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "setAllocation",
      "discriminator": [
        119,
        81,
        22,
        4,
        114,
        187,
        23,
        145
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "user",
          "type": "pubkey"
        },
        {
          "name": "allocation",
          "type": "u64"
        }
      ]
    },
    {
      "name": "unpauseContract",
      "discriminator": [
        111,
        62,
        159,
        38,
        222,
        74,
        12,
        133
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "unpauseSale",
      "discriminator": [
        99,
        124,
        44,
        17,
        154,
        84,
        167,
        103
      ],
      "accounts": [
        {
          "name": "sale",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "updateGlobalStats",
      "discriminator": [
        94,
        48,
        127,
        227,
        152,
        62,
        222,
        238
      ],
      "accounts": [
        {
          "name": "globalStats",
          "writable": true
        },
        {
          "name": "admin",
          "signer": true
        },
        {
          "name": "presale"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "globalStats",
      "discriminator": [
        119,
        53,
        78,
        3,
        254,
        129,
        78,
        28
      ]
    },
    {
      "name": "sale",
      "discriminator": [
        202,
        64,
        232,
        171,
        178,
        172,
        34,
        183
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "presalePaused",
      "msg": "La preventa está pausada."
    },
    {
      "code": 6001,
      "name": "notWhitelisted",
      "msg": "El usuario no está en la lista blanca."
    },
    {
      "code": 6002,
      "name": "allocationExceeded",
      "msg": "La cantidad solicitada excede la asignación."
    },
    {
      "code": 6003,
      "name": "calculationError",
      "msg": "Error en los cálculos."
    },
    {
      "code": 6004,
      "name": "invalidPresaleId",
      "msg": "ID de preventa inválido."
    },
    {
      "code": 6005,
      "name": "vestingPeriodNotEnded",
      "msg": "El período de vesting no ha terminado."
    },
    {
      "code": 6006,
      "name": "alreadyClaimed",
      "msg": "Ya has reclamado tus tokens."
    },
    {
      "code": 6007,
      "name": "noTokensToClaim",
      "msg": "No tienes tokens para reclamar."
    },
    {
      "code": 6008,
      "name": "insufficientSpace",
      "msg": "No hay suficiente espacio en la cuenta para añadir más entradas."
    },
    {
      "code": 6009,
      "name": "vestingAlreadyExists",
      "msg": "Vesting ya existe para este usuario."
    },
    {
      "code": 6010,
      "name": "tooManyPresales",
      "msg": "Se ha alcanzado el límite máximo de preventas simultáneas."
    },
    {
      "code": 6011,
      "name": "duplicatePresaleId",
      "msg": "ID de preventa duplicado."
    },
    {
      "code": 6012,
      "name": "invalidCommissionVault",
      "msg": "La cuenta de comisión no es válida."
    },
    {
      "code": 6013,
      "name": "presaleNotActive",
      "msg": "La preventa no está activa."
    },
    {
      "code": 6014,
      "name": "noPurchaseFound",
      "msg": "No se encontró ninguna compra para este usuario."
    },
    {
      "code": 6015,
      "name": "noVestingFound",
      "msg": "No se encontró información de vesting para este usuario."
    }
  ],
  "types": [
    {
      "name": "globalStats",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalRaised",
            "type": "u64"
          },
          {
            "name": "totalInvestors",
            "type": "u64"
          },
          {
            "name": "totalPresales",
            "type": "u32"
          },
          {
            "name": "presales",
            "type": {
              "vec": {
                "defined": {
                  "name": "presaleInfo"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "presaleInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "saleToken",
            "type": "pubkey"
          },
          {
            "name": "totalRaised",
            "type": "u64"
          },
          {
            "name": "totalInvestors",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "presaleStats",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "raiseToken",
            "type": "pubkey"
          },
          {
            "name": "saleToken",
            "type": "pubkey"
          },
          {
            "name": "totalTokens",
            "type": "u64"
          },
          {
            "name": "tokensSold",
            "type": "u64"
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "totalRaised",
            "type": "u64"
          },
          {
            "name": "totalInvestors",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "raiseGoal",
            "type": "u64"
          },
          {
            "name": "isActive",
            "type": "bool"
          },
          {
            "name": "timeRemaining",
            "type": "i64"
          },
          {
            "name": "percentageSold",
            "type": "f64"
          },
          {
            "name": "percentageRaised",
            "type": "f64"
          }
        ]
      }
    },
    {
      "name": "sale",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "totalTokens",
            "type": "u64"
          },
          {
            "name": "tokensSold",
            "type": "u64"
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "paused",
            "type": "bool"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "vestingEndTime",
            "type": "i64"
          },
          {
            "name": "raiseGoal",
            "type": "u64"
          },
          {
            "name": "totalInvestors",
            "type": "u32"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "userStats",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "allocation",
            "type": "u64"
          },
          {
            "name": "tokensPurchased",
            "type": "u64"
          },
          {
            "name": "vestingAmount",
            "type": "u64"
          },
          {
            "name": "vestingReleaseTime",
            "type": "i64"
          },
          {
            "name": "claimed",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
