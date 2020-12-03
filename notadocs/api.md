# NOTA API

```
TODO: Still not stable
```

- nota init
    - initializes the NOTA folders
    - Status: _Buggy_

- nota list
    - list available notas
    - Status: _TODO_

- nota new _NAME_
    - _NAME_ is optional
    - if _NAME_ is provided use it to create nota 
    - if not create a new one under Daily/<CurrentDay>
    - Status: _Buggy_

- nota add _PATH_
    - _PATH_ is required
    - adds an existing markdown file to the nota folder 
    - storing their metadata in the nota index
    - Status: _Buggy_

- nota update _NAME_
    - updates the NOTA by adding their metadata to the index
    - Status: _TODO_

- nota export _NAME_
    - _NAME_ is optional
    - exports the nota into the specific format, right now HTML
    - Status: _Buggy_