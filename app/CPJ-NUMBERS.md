# CPJ number budgets

The existing PROFILE2 number rule combines exact decimal **lexemes**, a 4096-digit budget and an inclusive absolute adjusted-exponent limit of 4096. This implementation interprets the digit budget as the number of written integer and fractional coefficient digits, excluding sign, decimal point and exponent notation. Written coefficient zeroes count toward that input allocation budget.

The adjusted exponent is computed after coefficient leading zeroes are removed; its inclusive range remains -4096 through +4096. Expanding an accepted exponent into plain decimal CPJ text is a separate operation: `1e4096` has a one-digit input coefficient and expands to 4097 digits; `1e-4096` expands to `0.` followed by 4095 zeroes and `1`. CPJ stores the normalized number text inside a typed number node. Its text need not itself fit the input-lexeme coefficient budget.

Tests cover the coefficient limits at 4096/4097 and the adjusted-exponent limits at both signs of 4096/4097, with complete expected CPJ number strings. This clarifies the existing implementation without changing the frozen PROFILE2 source, generation I33 or any of the nine golden payloads/digests. The interpretation is submitted for dsh to assess against the accepted contract; it does not silently redefine the frozen requirement as an expanded-output digit limit.
