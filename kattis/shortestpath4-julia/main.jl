using Distributed
using SharedArrays

# addprocs(4)

tests = parse(Int, readline())

function solve(vertices, s, t, k)
	dists = [typemax(Int64) for _ in 1:length(vertices)]
	dists[s] = 0
	reached = [(s, 0)]
	for round in 1:k
		next = []

		for (node, cost) in reached
			for (neigh, dist) in vertices[node]			
				new_dist = dist + cost
				if dists[neigh] > new_dist
					dists[neigh] = new_dist
					push!(next, (neigh, new_dist))
				end
			end
		end
		reached = next
	end

	if dists[t] == typemax(Int64)
		-1
	else
		dists[t]
	end
end

function solve2(vertices, s, ends, res)
	# print(first.(ends))
	maxk = maximum(first.(ends))
	end_map = Dict()
	for (k, t, qi) in ends
		kends = get!(end_map, k, [])
		push!(kends, (t, qi))
	end
	
	dists = [typemax(Int64) for _ in 1:length(vertices)]
	dists[s] = 0
	reached = [(s, 0)]
	for round in 1:maxk
		next = []

		for (node, cost) in reached
			for (neigh, dist) in vertices[node]			
				new_dist = dist + cost
				if dists[neigh] > new_dist
					dists[neigh] = new_dist
					push!(next, (neigh, new_dist))
				end
			end
		end
		reached = next

		# println(res)
		for (t, qi) in get(end_map, round, [])
			if dists[t] == typemax(Int64)
				res[qi] = -1
			else
				res[qi] = dists[t]
			end
		end
	end

end

problems = []
for test in 1:tests
	readline()
	
	vertices = []
	for _ in 1:parse(Int, readline())
		ints = [parse(Int, word) for word in split(readline())][2:end]
		push!(vertices, [(ints[i] + 1, ints[i+1]) for i in 1:2:length(ints)])
	end

	queries = Dict()
	nbrq = 0
	for qi in 1:parse(Int, readline())
		(s, t, k) = [parse(Int, word) for word in split(readline())]

		entry = get!(queries, s + 1, [])

		push!(entry, (k - 1, t + 1, qi))

		# push!(queries, [parse(Int, word) for word in split(readline())])
		nbrq = qi
	end

	push!(problems, (vertices, queries, nbrq))
end

prob_outs = [[] for _ in 1:length(problems)]
# @sync @distributed for ind in 1:length(problems)
for ind in 1:length(problems)
	(vertices, queries, nbrq) = problems[ind]
	res = [0 for _ in 1:nbrq]
	# @sync @distributed for (s, t, k) in queries
	for (s, ends) in queries
		# dist = solve(vertices, s+1, t+1, k-1)
		solve2(vertices, s, ends, res)
		# push!(res, dist)
	end
	prob_outs[ind] = res
end

for pres in prob_outs
	println.(pres)
	println()
end




