"12345"
	.map({ $1.@num() })
	.map({ $1 ** 2 })
	.filter({ $1 > 4 })
	# => [9, 16, 25]