&str name = &"josch"
name.* = "gaming"

void(&str) update = (to_update) {
    to_update.* = "shit"
}

update(name)

update = (to_update) {
    to_update.* = "fuck"
}

update(name)

@print("{name}")




