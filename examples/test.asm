
.section global
	display @0x400

.section text
	global _start

add:
	local a
	local b
	local r
	math.add a, b, r
	ret r

_start:
	local i
	set i, 0
	label loop
	copy i, ^.2
	copy &1, ^.3
	call ^, &add, display
	jump &loop
	ret null

