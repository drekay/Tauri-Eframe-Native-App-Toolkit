import os

def create_directory_structure():
    directories = [
        'src/lib/data_processing',
        'src/lib/file_management',
        'src/lib/external_integrations',
        'src/layouts',
        'src/windows',
        'src/components/ui',
        'src/plugins/ui_management',
        'src/core',
        'src/utils',
        'src/assets/images',
        'src/assets/icons',
        'src/styles',
    ]

    for directory in directories:
        os.makedirs(directory, exist_ok=True)
        print(f"Created directory: {directory}")

    # Create empty mod.rs files
    mod_rs_locations = [
        'src/lib',
        'src/lib/data_processing',
        'src/lib/file_management',
        'src/lib/external_integrations',
        'src/plugins',
        'src/plugins/ui_management',
    ]

    for location in mod_rs_locations:
        mod_rs_path = os.path.join(location, 'mod.rs')
        if not os.path.exists(mod_rs_path):
            with open(mod_rs_path, 'w') as f:
                pass  # Create an empty file
            print(f"Created empty mod.rs file: {mod_rs_path}")

if __name__ == "__main__":
    create_directory_structure()