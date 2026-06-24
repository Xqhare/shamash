- [ ] Removal of all `panics`, `unwraps` or `expects` ::
    * Instead log the errors in an error file (`base_dir/error.log`)
- [ ] Fix several major limitations ::
    - [ ] Logs of ongoing incidents are not saved :: and kept in heap allocated memory. Not only not performant, but also should the system crash, nothing would be saved
    - [ ] Log files are not rotated :: 
    - [ ] Add a json file :: for configuration without docker or touching the source code
    - [ ] Logger helper function :: for entries with a date & time (unifies formatting)

