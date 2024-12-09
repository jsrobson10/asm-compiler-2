
.section data
	digits 0b11101110, 0b00100100, 0b10111010, 0b10110110, 0b01110100, 0b11010110, 0b11011110, 0b10100100, 0b11111110, 0b11110110
	digit_minus 0b00010000
	digit_dot   0b00000001

.section global
	tmp 1

.section text
	global main

display_digit:
	local v
	local offset
	local p_bits
	local p_display
	math.add v, &digits, p_bits
	math.add offset, &0x420, p_display
	copy_ls p_bits, p_display
	ret null

display_number:
	local v

	local at
	set 8, at
	set 0, 0x431 ; clear screen

	label loop
	math.sub at, &1, at
	copy at, ^+3
	math.mod v, &10, ^+2
	math.div v, &10, v
	call &^, &display_digit, null
	jump_z at, &end
	jump_if v, &loop
	label end
	set 0, 0x432 ; sync screen
	ret null

main:
	local at
	set 0, at
	label loop

	math.add at, &1, at
	copy at, ^+2
	call &^, &display_number, null
	jump &loop
	stop

