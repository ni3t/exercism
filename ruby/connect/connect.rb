class Board
  
  # @@board = [
  #   '0 0 0 X',
  #   ' X . . X',
  #   '  X . . X',
  #   '   X 0 0 0',
  # ].map{|row| row.split(" ")}

  
  
  Field = Struct.new(:x,:y,:player) {
    def neighbors
      -> {
        @@fields.flatten.select do |f|
          ->a{a.zip(a.map(&:reverse))}[[-1,1].product([0])]
            .flatten(1).concat([[-1,1],[1,-1]])
            .include?([(self.x-f.x),(self.y-f.y)])
        end
      }
    end
  }
  
  attr_reader :string_arr, :height, :width, :fields
  
  def initialize(string_arr)
    @@board = string_arr.map{|r| r.split(" ")}
    puts "\n------------------\n\n"
    puts @@board.zip(0.step).map{|y,i| (" "*i) + y.join(" ")}.join("\n")
    @height = string_arr.length
    @width = string_arr.first.split(" ").length
    @@fields = Array.new(@height){|y| Array.new(@width){|x| Field.new(x+1,y+1,@@board[y][x]) }}
    puts "\n------------------\n\n"
  end

  def winner
    @res = @@fields.first.select{|f| f.player != "."}.map{|f| @chain=[];try_paths(f)}.flatten.first.to_s
    puts "first result: #{@res=='' ? "No winner." : @res}"
    if @res=='' 
      @@board = @@board.transpose
      puts "\n------------------\n\n"
      puts @@board.zip(0.step).map{|y,i| (" "*i) + y.join(" ")}.join("\n")
      @height = @@board.length
      @width = @@board.first.length
      @@fields = Array.new(@height){|y| Array.new(@width){|x| Field.new(x+1,y+1,@@board[y][x]) }}
      puts "\n------------------\n\n"
      @chain = []
      @res = @@fields.first.select{|f| f.player != "."}.map{|f| try_paths(f)}.flatten.first.to_s
      puts "second result: #{@res=='' ? "No winner." : @res}"
    end
    if @res!=''
      puts "Winner: #{@res}"
    else
      puts "No winner found."
    end
    puts "\n------------------\n\n"
    @res
  end
  
  def try_paths field
    return if field.nil?
    @chain ||= []
    @rejected ||=[]
    @chain << field
    if field.y == height
      puts "winning chain is #{@chain.map{|f| "#{f.x},#{f.y}"}}"
      return field.player
    end  
    valid_neighbors = field.neighbors.call.select do |f| 
      (f.player != ".") && (f.player == field.player) && !@chain.member?(f) && !@rejected.member?(f)
    end
    valid_neighbors.empty? &&
      @rejected << @chain.pop &&
      @chain.pop
    valid_neighbors.map{|f| try_paths(f)}
  end
  

end