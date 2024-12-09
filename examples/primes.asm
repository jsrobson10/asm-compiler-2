
.section data
	display 0x400
	primes_max 32

.section global
	primes 32
	primes_count 1

	prime_at 1
	tmp 1

.section text
	global main

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
	set 0, 0x430
	label next
	math.add tmp, display, tmp
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

	call &^, &add_prime, tmp
	jump_if tmp, &loop
	ret null
	
