#!/usr/bin/env python3

import os
import subprocess
import shutil
from pathlib import Path


print("Welcome to Cosmic Application Template")
name = input("Name (e.g. My Awesome App): ")
project_name = input("Project Name (e.g. my-awesome-app): ")
app_id = input(
    "Application ID (e.g. org.domain.MyAwesomeApp, see: https://developer.gnome.org/documentation/tutorials/application-id.html ): "
)
for segment in app_id.split(".")[:-1]:
    if "-" in segment:
        exit("App Id might only contain '-' in the last segment")

author = input("Author: ")
update_contact = input("Email: ")

project_name_alt = project_name.replace("-", "_")

app_path = "/" + "/".join(app_id.split(".")) + "/"
project_dir = Path(__file__).parent / project_name

CURRENT_APP_ID = "com.system76.CosmicApplicationTemplate"
CURRENT_PROJECT_NAME = "cosmic-application-template"
CURRENT_PROJECT_NAME_ALT = "cosmic_application_template"
CURRENT_NAME = "Cosmic Application Template"
CURRENT_AUTHOR = "Ashley Wulber"
CURRENT_EMAIL = "ashley@system76.com"
CURRENT_APP_PATH = "/com/system76/CosmicApplicationTemplate/"


if project_dir.is_dir():
    wanna_remove = ""
    while wanna_remove not in ["y", "n"]:
        wanna_remove = input(
            "Project already exists, do you want to remove it? [y/n] "
        ).lower()

    if wanna_remove == "y":
        shutil.rmtree(project_dir)
    else:
        exit()


items_to_copy = [
    Path(".github"),
    Path("build-aux"),
    Path("data"),
    Path("hooks"),
    Path("po"),
    Path("src"),
    Path(".editorconfig"),
    Path(".gitignore"),
    Path(".gitlab-ci.yml"),
    Path("Cargo.toml"),
    Path("build.rs"),
    Path("meson_options.txt"),
    Path("meson.build"),
    Path("README.md"),
    Path("i18n"),
    Path("i18n.toml"),
    Path("debian"),
]

for item in items_to_copy:
    item_path = Path(item)
    if item_path.is_dir():
        shutil.copytree(item_path, project_dir / item)
    else:
        shutil.copyfile(item_path, project_dir / item)


files_with_content_to_rename = [
    Path("build-aux") / "com.system76.CosmicApplicationTemplate.Devel.json",
    Path("data") / "com.system76.CosmicApplicationTemplate.desktop.in.in",
    Path("data") / "com.system76.CosmicApplicationTemplate.gschema.xml.in",
    Path("data") / "com.system76.CosmicApplicationTemplate.metainfo.xml.in.in",
    Path("data") / "resources" / "resources.gresource.xml",
    Path("po") / "POTFILES.in",
    Path("src") / "main.rs",
    Path("src") / "components" / "example.rs",
    Path("Cargo.toml"),
    Path("meson.build"),
    Path("meson_options.txt"),
    Path(".gitlab-ci.yml"),
    Path(".github") / "workflows" / "ci.yml",
    Path("i18n") / "en" / "cosmic_application_template.ftl",
    Path("debian") / "changelog",
    Path("debian") / "control",
    Path("debian") / "copyright",
    Path("debian") / "rules",
]

for file in files_with_content_to_rename:
    current_path = project_dir / file

    with open(current_path, "r") as handle:
        content = handle.read()
        content = content.replace(CURRENT_APP_ID, app_id)
        content = content.replace(CURRENT_APP_PATH, app_path)
        content = content.replace(CURRENT_PROJECT_NAME, project_name)
        content = content.replace(CURRENT_PROJECT_NAME_ALT, project_name_alt)
        content = content.replace(CURRENT_NAME, name)
        content = content.replace(CURRENT_AUTHOR, author)
        content = content.replace(CURRENT_EMAIL, update_contact)

    with open(current_path, "w") as handle:
        handle.write(content)

files_to_rename = [
    Path("build-aux") / "com.system76.CosmicApplicationTemplate.Devel.json",
    Path("data") / "icons" / "com.system76.CosmicApplicationTemplate-symbolic.svg",
    Path("data") / "icons" / "com.system76.CosmicApplicationTemplate.svg",
    Path("data") / "icons" / "com.system76.CosmicApplicationTemplate.Devel.svg",
    Path("data") / "com.system76.CosmicApplicationTemplate.desktop.in.in",
    Path("data") / "com.system76.CosmicApplicationTemplate.gschema.xml.in",
    Path("data") / "com.system76.CosmicApplicationTemplate.metainfo.xml.in.in",
    Path("i18n") / "en" / "cosmic_application_template.ftl"
]


for file in files_to_rename:
    current_path = project_dir / file
    new_path = project_dir / file.parent / str(file.name).replace(CURRENT_APP_ID, app_id).replace(CURRENT_PROJECT_NAME_ALT, project_name_alt)
    shutil.move(current_path, new_path)


subprocess.call(["git", "init"], cwd=project_dir)
# Add all files and commit them
subprocess.call(["git", "add", "-A"], cwd=project_dir)
subprocess.call(["git", "commit", "-m", "Init with GTK Rust Template"], cwd=project_dir)
