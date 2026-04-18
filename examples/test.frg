&int var = &5
void(&int) increment = (to_update) {
	to_update.* += 1
}
increment(var)
