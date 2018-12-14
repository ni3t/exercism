class BookStore

  def self.calculate_price basket
    # puts "\n----------------------\n\n"
    # # sum_book_combo(unique_basket_combos(basket)[2])

    pp combos = unique_basket_combos(basket)
    
    # combos.map{|ubc|sum_book_combo(ubc)}.min
    # puts "\n----------------------\n\n"
    # unique_basket_combos(basket).map{|ubc|sum_book_combo(ubc)}.min || 0.0
  end


  def self.unique_basket_combos b
    # num = (b.length/2 - 1) > 0 ? (b.length/2 - 1) : 1
    # puts "finding permutations that have arrays of #{num} to #{b.length} length for #{b}" 
    # final = [*(num..b.length)].map do |len|
    #   # puts "now looking at #{len} #{Time.now}"
    #   b.permutation(len).to_a.map{|a| 
    #     [*(1..a.length)]
    #       .map{|i| 
    #         a.each_slice(i).to_a.map(&:sort).reject{|c| c.map{|d| c.count(d) > 1}.any?}
    #       }
    #     }.flatten(1).map(&:sort).uniq.reject{|e| 
    #       e.flatten.count != b.count
    #     }
    #   end.flatten(1)
    # puts final.map{|g| g.map{|f| f.join(",")}.join(" | ")}.join("\n\n")
    b.uniq.map{|c| b.count(c)}.max
    # final
  end

  # def self.sum_book_combo aa
  #   final = aa.map{|bb| bb.count * (determine_discount(bb.count)*8)}.sum
  #   # pp "#{aa} -- $#{sprintf('%.2f',final)}"
  #   final
  # end

  # def self.determine_discount count
  #   count <= 3 ? 
  #     ((20 - (count - 1)) * 0.05).round(2) :
  #     ((20 - (count)) * 0.05).round(2)
  # end




end