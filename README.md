# TODO

  - proper error handling. Trigger all ? errors, and verify proper error messages and context are present
  - we had a situation where we had the default root key in our userset config file. No errors are thrown, but silently the properties
    are not what you expect them to be. We should probably not allow mergin in unexisting keys. Userset should only be allowed to over
    ride existing keys from default. Make sure we have a decent error message for this too!

  - consider deriving the custom settings object from defaults.yml where named objects will be of type that name, and unnamed objects
    will be a serde_yaml type.
    - Advantage: less typing
    - disadvantage: not so clear where types come from as it's a macro

  - rethink merge_runtime and merge_userset. Should we only accept yaml strings or is another data format more appropriate.
  - documentation
  - clean up reported errors, provide context
  - .d directories
  - write out configuration to file
