class Trinary
  attr_reader :i

  def initialize i
    @i=i
  end
  ###
  #
  #   score: 68 (4 under par) 
  # 
  #   1. check to see if string is valid, otherwise return 0
  #   2. convert string to arr of chars
  #   3. reverse the string
  #   4. map with index
  #     a. char * 3 ^ index == representation
  #     b. sum them all up with inject(&:+)
  # 
  ###
  def to_decimal
    i=~/[^012]/?0:i.chars.reverse.zip(0.step).map{|e,i|e.to_i.*3**i}.sum

  end
end