require "./assemblage.rb"

null = Assemblage.new
p null

a = Assemblage.new(name: "Charlie")
b = Assemblage.new(name: "Bob")
c = Assemblage.cons(a, b, "Charlie Bob Friendship", 100.0)
p c
a[:intensity] = 42.0
p c

coll = [null, null, null].reduce(null) do |sum, x|
  Assemblage.cons(sum, x, "(cons #{sum[:name]} #{x[:name]})", 0.0)
end
  
Assemblage.each(coll) do |a|
  p a
end

names = Assemblage.map(coll) do |a|
  a[:name]
end

p names

Assemblage.update(a, 42.0)
p a

Assemblage.each(coll) do |a|
  Assemblage.update(a, 42.0)
end

intensities = Assemblage.map(coll) do |a|
  a[:intensity]
end

p intensities
