
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

	set a, 0
	set b, 1
	set c, 0
	set counter, 0

	label loop
	math.add a, b, c
	copy b, a
	copy c, b
	math.add display, counter, tmp
	store a, tmp
	math.lthan counter, &16, tmp
	jump_if tmp, &loop
	ret null

