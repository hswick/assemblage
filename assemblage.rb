module Assemblage
  
  def Assemblage.new(pair: [], intensity: 0.0, name: "Null")
    {pair: pair, intensity: intensity, name: name, trajectory: [intensity]}
  end

  def Assemblage.cons(a, b, name=nil, intensity=0.0)
    name = a[:name] + " + " + b[:name] if name.nil?
    Assemblage.new(pair: [a, b], intensity: intensity, name: name)
  end

  def Assemblage.update(a, intensity)
    a[:intensity] = intensity
    a[:trajectory] << intensity
  end

  def Assemblage.first(a)
    a[:pair][0]
  end

  def Assemblage.second(a)
    a[:pair][1]
  end

  def Assemblage.each(a)
    f = Proc.new
    if a[:name] == "Null" || a[:pair] == []
      f.call(a)
    else
      f.call(a)
      each(Assemblage::first(a)) { |x| f.call(x) }
      each(Assemblage::second(a)) { |x| f.call(x) }
    end
    nil
  end
  
  def Assemblage.map(a)
    f = Proc.new
    output = []
    each(a) do |x|
      output << f.call(x)
    end
    output
  end

  def Assemblage.empty?(a)
    a[:pair].length == 0
  end

end
