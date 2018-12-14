class Hexadecimal
  def initialize string
    @s = string
  end

  def to_decimal
    # check to see if the string is valid, otherwise return 0
    # reverse the string and convert to array
    # each character, if c -> to_s -> to_i == c then it is an integer, otherwise it is a character
    # if a character, subtract 87 from its bytes ("a" -> 97, "f" -> 102) to get its base value ("a" -> 10, "f" -> 15)
    # base * 16 ^ index == value
    # sum all values
    @s=~/[^0-f]/ ? 0 : @s.chars.reverse.each_with_index.map{|e,i|e.to_i.to_s==e ? [e,i] : [e.bytes[0]-87,i]}.map{|e,i|e.to_i*(16**i)}.inject(&:+)
  end
end