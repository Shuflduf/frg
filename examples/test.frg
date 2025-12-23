set(int) winners = {2, 4, 5}
set(int) tall = {2, 5, 7}
set(&int) tall_winners = winners.intersection(&tall).collect()
@print("{tall_winners:?}")




