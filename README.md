A repository for libraries and applications related to generating experiment coverage information. Previously these had all been in separate repositories:

- [exp_viz](https://github.com/ReddyLab/exp_viz) - library for loading and filtering coverage data
- [cov_viz](https://github.com/ReddyLab/cov_viz) - application for generating coverage data
- [cov_viz_ds](https://github.com/ReddyLab/cov_viz_ds) - common data structures
- [cov_viz_manifest](https://github.com/ReddyLab/cov_viz_manifest) - application for generating the "manifest" file read by the web application

The only part that isn't here is the library that wraps exp_viz for django to use. That is in [the portal's repository](https://github.com/ReddyLab/cegs-portal/tree/main/extensions/exp_viz)

Additionally, an app for testing code changes, filter_viz_test, is included.
