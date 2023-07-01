import os
import sys
from content import (
    first_mod_rs_content,
    controller_rs_content,
    dto_rs_content,
    service_rs_content,
    database_rs_content,
)

if not os.path.exists("src/domains"):
    print("The src/domains directory does not exist. Terminating script.")
    sys.exit(1)

# continue with the rest of the script here
folder_name = input("Enter the name of the folder to create in src/domains: ")
dto_name = input("Enter the name of the dto: ")
# if the dto name doesn't not start with the Capital letter, then capitalize the first Letter
if not dto_name[0].isupper():
    dto_name = dto_name.capitalize()
data_base_name = input("Enter the name of the database table: ")
# go into the src/schema.rs and if there is no table named data_base_name, then throw an error
# if there is a table named data_base_name, then continue with the rest of the script

folder_path = os.path.join("src", "domains", folder_name)

if not os.path.exists(folder_path):
    os.makedirs(folder_path)
    print(f"Created folder: {folder_path}")
else:
    print(f"Folder already exists: {folder_path}")


# Create the controllers, dto, and services folders
for subfolder in ["controllers", "dto", "services"]:
    subfolder_path = os.path.join(folder_path, subfolder)
    if not os.path.exists(subfolder_path):
        os.makedirs(subfolder_path)
        print(f"Created folder: {subfolder_path}")

# Create the mod.rs and module.rs files

for filename, content in [("mod.rs", first_mod_rs_content), ("module.rs", "")]:
    file_path = os.path.join(folder_path, filename)
    with open(file_path, "w") as f:
        f.write(content)
        print(f"Created file: {file_path}")


# Create the {input}_controller.rs and mod.rs files inside the controllers folder
controllers_folder = os.path.join(folder_path, "controllers")
mod_rs_content = f"pub mod {folder_name}_controller;\n"

for filename, content in [
    (f"{folder_name}_controller.rs", controller_rs_content(folder_name)),
    ("mod.rs", mod_rs_content),
]:
    file_path = os.path.join(controllers_folder, filename)
    with open(file_path, "w") as f:
        f.write(content)
        print(f"Created file: {file_path}")

# inside the dto folder, create the {input}_dto.rs and mod.rs files
# inside the mod.rs file, add the line: pub mod {input}_dto;
# inside the {input}_dto.rs file, add the following content:
# dto_rs_content(dto_name, data_base_name)

dto_folder = os.path.join(folder_path, "dto")
mod_rs_content = f"pub mod {folder_name}_dto;\n"

for filename, content in [
    (f"{folder_name}_dto.rs", dto_rs_content(dto_name, data_base_name)),
    ("mod.rs", mod_rs_content),
]:
    file_path = os.path.join(dto_folder, filename)
    with open(file_path, "w") as f:
        f.write(content)
        print(f"Created file: {file_path}")

# inside the services folder, create the {input}_service.rs and mod.rs files and one folder named database
# inside the mod.rs file, add the line: pub mod {input}_service; pub mod database;
# inside the {input}_service.rs file, add the following content: service_rs_content(folder_name)
# inside the database folder, create the {input}_database.rs and mod.rs files

services_folder = os.path.join(folder_path, "services")
mod_rs_content = f"pub mod {folder_name}_service;\npub mod database;\n"
for filename, content in [
    (f"{folder_name}_service.rs", service_rs_content(folder_name)),
    ("mod.rs", mod_rs_content),
]:
    file_path = os.path.join(services_folder, filename)
    with open(file_path, "w") as f:
        f.write(content)
        print(f"Created file: {file_path}")

# make the database folder
database_folder = os.path.join(services_folder, "database")
if not os.path.exists(database_folder):
    os.makedirs(database_folder)
    print(f"Created folder: {database_folder}")

# inside the database folder, create the {input}_database.rs and mod.rs files

mod_rs_content = f"pub mod {folder_name}_database;\n"
for filename, content in [
    (f"{folder_name}_database.rs", database_rs_content(folder_name)),
    ("mod.rs", mod_rs_content),
]:
    file_path = os.path.join(database_folder, filename)
    with open(file_path, "w") as f:
        f.write(content)
        print(f"Created file: {file_path}")
