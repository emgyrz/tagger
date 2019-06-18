# tagger


[![npm](https://img.shields.io/npm/v/tagger-bin.svg)](https://www.npmjs.com/package/tagger-bin)

Simple utility to do something (e.g. install) packages from git repository by selecting git tag which is presented by semver rules.

Tagger works with git repositories only through ssh connection.

If you have a package that is not in npm or local repos but it has versionong, `tagger` may be helpful for you.


### Requirements
`tagger` works with:
  - ~~macOS~~ `// TODO`
  - Linux (64-bit)
  - Windows (64-bit)



### CLI
```
Usage: tagger (--help | --version)
       tagger [--cfg PATH] (--show-latest | --list-all) PACKAGES...
       tagger [--exec --cfg PATH] PACKAGES...

Options:
    -h, --help              Show this message.
    -v, --version           Show the version of tagger.
    -c PATH, --cfg PATH     Path to config file. If not specified config will be searching in ./.tagger.cfg.json and ~/.tagger.cfg.json"
    -l, --show-latest       Prints latest valid tag.
    -a, --list-all          Prints all valid by semver tags.
    -e, --exec              *optional. Do something with specified or latest package version.

Examples:
    tagger ui
    tagger --show-latest hlp
    tagger --exec -c ../path_to_tagger_config.json hlp@2.1.3
```

### Config file
Config file name by default is `.tagger.cfg.json`. And if it is not specified, it is searched in directory where you run `tagger` or in your home directory (e.g. `~/.tagger.cfg.json`).

Config includes fields:
#### `repos`
Type: `Array<{name: string, url: string}>`

List of package repositories you want to use. Each item in this list cantains `name` ( to identify it )
and `url` know how to connect.
Example:
```js
{
  "repos": [
    {
      "name": "tagger",
      "url": "ssh://git@github.com:emgyrz/tagger.git"
    }
  ],
  // ...
}
```

#### `command`
Type: `String`

Says what to do with package. In `command` you have several variables that will be replaced on executing
- `{NAME}` - name of package presents in repo
- `{URL}` - same
- `{VERSION}` - version of package you were specify or latest version

Example:
```js
{
  // ...
  "command": "yarn add {URL}#{VERSION}"
  // "command": "echo 'my latest version of package {NAME} is {VERSION}'",
  // "command": "rm -rf / :)"
}
```


##### Enjoy using!

### License

This package is [MIT licensed](./LICENSE).
