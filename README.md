# NOTA

NOTA is a simple note taker tool that helps you organize your notes.

# Why Another tool ? Aren't there enough already ?

Yeah, there are and that is part of the problem NOTA aims to fix.
I wanted to write my notes like I like to write my code: with no predefined 
editor, only tooling around the language.
I wanted to be this way for my notes too, I wanted to be able to edit and take
notes with any editor I wanted. No app lock in's no enforced programs or shaddy
web apps to use to do the things I wanted.

Simple text files, with a litle bit of tooling around for the extra producivity

**Very early state in the project, not ready in any way shape or form xD**

# Features

- Basic note management (create, update list, search)
- Linking and Reverse Linking of notes
- Export to other formats

# Install 

- Download the binary
- Add it to your path
- Create a folder for your notes (optional)
- Add a Environment Variable NOTA_FOLDER that points to the folder
- You are good to go !

# API

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