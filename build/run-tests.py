#!/usr/bin/python3

import itertools
import subprocess
import sys

if __name__ == "__main__":
    VAPOURSYNTH_FUNCTIONS = ["vapoursynth-functions"]
    VSSCRIPT_FUNCTIONS = ["vsscript-functions"]
    F16_PIXEL_TYPE = ["f16-pixel-type"]

    features = [
        VAPOURSYNTH_FUNCTIONS,
        VSSCRIPT_FUNCTIONS,
        F16_PIXEL_TYPE,
    ]

    for f in features:
        f += [""]

    for features in itertools.product(*features):
        features_string = str.join(" ", features)
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
