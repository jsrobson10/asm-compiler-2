
.section data
	display 0x400

.section global
	tmp 1

.section text
	global main

main:
	local a
	local b
	local c
	local counter

	set 0, a
	set 1, b
	set 0, counter

	label loop
	math.add a, b, c
	copy b, a
	copy c, b
	math.add display, counter, tmp
	store a, tmp
	math.add counter, &1, counter
	math.lthan counter, &16, tmp
	jump_if tmp, &loop
	ret null

