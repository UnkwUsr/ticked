# TODO's for ticked

## bugs

* handle bad responses on bad user cookie
* when exit with ctrl-c on prompt "type enter to continue", tempfile is not
  deleted and remains in tmp

## refactor

* as lines identifier can use markdown numbered lists. So number of item will
  be identifier of task
  * it will solve many problems and even allow to drop division into modes
    "delete"/"edit"
* drop markdown parser lib. It's not suit for my case. maybe

## feat

* support metadata: tags, date
  * and sort by them, categorize separately
* support more lists (not only main "inbox")
  * then also ability to move tasks between different lists
  * for example, can pass lists names in command line arguments

### later

* maybe cache tasks, so for future requests have to load only changed tasks?
* textwrap
  * <https://github.com/mgeisler/textwrap>

### useless

* delete mode: maybe add "reorder" feature?
