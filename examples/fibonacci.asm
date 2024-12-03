
.section data
	display 0x400

.section global
	tmp 1

.section text
	global main

main:
	local_set a, 0
	local_set b, 1
	local_set c, 0
	local_set counter, 0
	label loop
	math.add a, b, c
	copy b, a
	copy c, b
	math.add display, counter, tmp
	store a, tmp
	math.lthan counter, &16, tmp
	jump_if tmp, loop
	ret

