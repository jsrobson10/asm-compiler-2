
.section global
	display @0x400

.section text
	global _start

_start:
	local_set i, 0
	label loop
	math.add i, &1, i
	copy i, display
	jump &loop
	ret

