# Submitting a Pull Request

## Pretext

![plsmerge](https://i.redd.it/n6peul2y3so01.png)

Merging or closing of pull requests is solely at the discretion of the owners of the repository. Common reasons for immediate closure of pull requests include, but are not limited to:

- Dead code
- Inefficient resource use
- Code with limited use
- Improper formatting
- Inappropriate content

## Formatting

### Rust

All Rust code must abide by the proper formatting of the official Rust Formatting [RFC 1607.](https://github.com/rust-lang/rfcs/pull/1607). For detailed information how the styling works you can read the [formatting guide](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md).

### Ruby

Anything written in Ruby must follow the style guide as documented [here](https://github.com/bbatsov/ruby-style-guide) and enforced by [RuboCop](https://github.com/bbatsov/rubocop).

### Python

Make sure that your code is formatted according to [PEP8](https://www.python.org/dev/peps/pep-0008/).

Your Editor/IDE probably comes with a linter installed or has plugins for linting available.

You can also use a CLI tool like `flake8` to manually lint your code.


## Commit Messages

Keep your commit messages short and informative.
You can add a more comprehensive description below the commit message.

Things to **not** include in your commit message:

* `@tag` team or username references
* `#ref` issue or pull request references

Never include `[ci skip]` or similar to skip CI.
Pull requests skipping any kind of CI will be ignored.

Check out this [article](https://chris.beams.io/posts/git-commit) if you want to know why good commit messages matter.
