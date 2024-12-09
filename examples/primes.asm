
.section data
	digits 0b11101110, 0b00100100, 0b10111010, 0b10110110, 0b01110100, 0b11010110, 0b11011110, 0b10100100, 0b11111110, 0b11110110
	primes_max 64

.section global
	primes 64
	primes_count 1

	prime_at 1
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

get_next:
	local candidate
	copy prime_at, candidate
	math.add prime_at, &2, prime_at
	ret candidate

is_prime:
	local candidate

	local prime
	local i
	set 0, i

	label loop
	math.lthan i, primes_count, tmp
	jump_z tmp, &end
	math.add &primes, i, tmp
	load tmp, prime
	math.mod candidate, prime, tmp
	jump_if tmp, &next
	ret &0

	label next
	math.add i, &1, i
	jump &loop

	label end
	ret &1

add_prime:
	local prime

	math.add primes_count, &primes, tmp
	store prime, tmp
	math.and primes_count, &15, tmp
	jump_if tmp, &next
	set 0, 0x430 ; clear screen
	label next
	math.add tmp, &0x400, tmp
	store prime, tmp
	math.add primes_count, &1, primes_count
	math.lthan primes_count, primes_max, tmp
	ret tmp

main:
	local prime
	set 0, primes_count
	set 3, prime_at

	label loop

	call &^, &get_next, prime

	copy prime, ^+2
	call &^, &is_prime, tmp
	jump_z tmp, &loop

	copy prime, ^+2
	call &^, &display_number, null

	copy prime, ^+2
	call &^, &add_prime, tmp
	jump_if tmp, &loop
	ret null
	
