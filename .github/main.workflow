workflow "New workflow" {
  on = "push"
  resolves = ["new-action"]
}

action "rust" {
  uses = "docker://rust:1"
  runs = ".travis/build.sh"
}

action "new-action" {
  uses = "owner/repo/path@ref"
  needs = ["rust"]
}
