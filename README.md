# empty-project

## Usage

Clone a repository from specific branch (project template) into new directory:

`git clone https://github.com/sergey-melnychuk/empty-project.git -b maven-java8-junit5 new-project-name`

Don't forget to remove the link to this repository and template's commit history:

`cd new-project-name && rm -rf .git`

At this point you have runnable template where you just need to rename project and packages.

## Templates

- `maven-java8-junit5`
- `sbt-scala2.13-play2.7
- `bazel-cpp-gtest`

