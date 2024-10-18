import os

def create_directory_structure():
    base_dir = os.getcwd()  # Get the current working directory
    
    # Define the structure
    structure = {
        'src': {
            'utils': ['math_helpers.rs', 'ui_helpers.rs'],
            'windows': ['main_window.rs', 'secondary_window.rs', 'settings_window.rs'],
            'lib.rs': None,
            'main.rs': None
        },
        'scripts': ['build.sh', 'test.sh'],
        'tools': ['code_generator.rs', 'performance_profiler.rs'],
        'templates': {
            'project_structures': ['basic_window_app.rs'],
            'configurations': ['default_config.toml']
        },
        'assets': {
            'images': None,
            'icons': None,
            'fonts': None
        },
        'tests': {
            'unit': None,
            'integration': None
        },
        'docs': {
            'api': None,
            'user_guide': None
        },
        'examples': ['basic_app.rs', 'complex_app.rs']
    }

    def create_structure(current_path, structure):
        for key, value in structure.items():
            path = os.path.join(current_path, key)
            if isinstance(value, dict):
                os.makedirs(path, exist_ok=True)
                create_structure(path, value)
            elif isinstance(value, list):
                os.makedirs(path, exist_ok=True)
                for file in value:
                    open(os.path.join(path, file), 'a').close()
            elif value is None:
                if os.path.splitext(key)[1]:  # If it's a file (has an extension)
                    open(os.path.join(current_path, key), 'a').close()
                else:
                    os.makedirs(path, exist_ok=True)

    create_structure(base_dir, structure)
    print("Directory structure created successfully!")

if __name__ == "__main__":
    create_directory_structure()