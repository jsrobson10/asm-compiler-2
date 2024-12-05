
.section data
	display 0x400

.section global
	tmp 1

.section text
	global main

main:
	local a, 2
	local b, 2
	local c, 2
	local counter

	set a+0, 0
	set b+0, 1
	set c+0, 0
	set a+1, 0
	set b+1, 0
	set c+1, 0
	set counter, 0

	label loop
	math.add a+0, b+0, c+0
	math.addc a+1, b+1, c+1
	copy b+0, a+0
	copy c+0, b+0
	copy b+1, a+1
	copy c+1, b+1
	math.add display, counter, tmp
	store a+0, tmp
	math.add tmp, &1, tmp
	store a+1, tmp
	math.add counter, &2, counter
	math.lthan counter, &16, tmp
	jump_if tmp, &loop
	ret null

