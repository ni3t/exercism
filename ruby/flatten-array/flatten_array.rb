class FlattenArray
  
  def self.flatten n
    f(n)
  end
  
  ###
  #
  #   13 under par (59 bytes)
  #
  #   1. proc takes an empty array
  #   2. maps over the potentially nested array
  #   3. each element:
  #     a. check if it's an array by subtracting it from itself and comparing to an empty array
  #     b. if it is an array, it loops through children, then adds each child of the nested return to the proc's passed in array
  #     c. if it is not an array, it checks to see if it is nil, if not nil then add to proc's passed in array
  #
  ###
  def self.f n
    ->a{n.map{|e|e&&e-e==[]?f(e).map{|b|a<<b}:(e&&a<<e)};a}[[]]
  end

end