#!/usr/bin/python3

import itertools
import subprocess
import sys
import argparse

if __name__ == "__main__":
    F16_PIXEL_TYPE = ["f16-pixel-type"]

    R73_COMPAT_FEATURE = "vsscript-r73-compat"
    R73_COMPAT = [R73_COMPAT_FEATURE]

    parser = argparse.ArgumentParser()
    parser.add_argument('--requires-compat')
    args = parser.parse_args()
    args.requires_compat = args.requires_compat == "true"

    features = [
        F16_PIXEL_TYPE,
        R73_COMPAT
    ]

    for f in features:
        f += [""]

    feature_combinations: set[str] = set()
    for tested_features in itertools.product(*features):
        tested_features = set(tested_features)

        # unnecessary after creating 
        if "" in tested_features:
            tested_features.remove("")

        if args.requires_compat and R73_COMPAT_FEATURE not in features:
            tested_features.add(R73_COMPAT_FEATURE)

        features_string = str.join(" ", tested_features).strip()
        feature_combinations.add(features_string)

    for features_string in feature_combinations:
        print("Starting tests with features: " + features_string)
        sys.stdout.flush()

        try:
            subprocess.run(
                ["cargo", "test", "--verbose", "--features", features_string],
                check=True,
            )
        except subprocess.CalledProcessError:
            print(features_string + " failed. Exiting with code 1.")
            sys.exit(1)
