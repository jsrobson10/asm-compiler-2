
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
	local_set a, 5
	local_set b, 7
	local r
	push a
	push b
	call add, display
	ret

