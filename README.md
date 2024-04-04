# venom
A python project management tool written in Rust


### Commands
1. `venom init <project_name>`
    - Create a python project with a an env set up
    - The enviroment will already be activated (I need to think about env dir location)
2. `venom install <package_name>`
    - This will use pip to install the specified package
    - It will activate the associated env for the project and install in that project
3. `venom run`
    - No idea how this will work
    - Mabye a main.py file
4. `venom test`
    - Will run all the project tests
    - Again, no idea how I want this to work
