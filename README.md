# StarWarsGenderProject
Using the Star Wars social network graph available on Kaggle, I quantify gender representation in the first seven Star Wars films by calculating the percentage of interactions that are between two women, two men, and other combinations. I also calculate the degree of separation between female characters using breadth-first search.

This code is based on the Star Wars social network dataset available on Kaggle under user Ruchi Bhatia. This project uses only the graphs whose titles are of the form "episode-#-interactions-allCharacters.json," though you could substitute any other dataset and it should work. For this project, I edited the datasets for episodes 2 and 4 by removing characters that were in the script, but did not appear in the final cut. I also deleted a link between Padme and Beru in episode 2 that did not appear in the final cut of the movie.

Gender data was not available for the characters in these graphs, so I created a csv file with each character's name and gender called "starwars_genders.csv." The test module makes sure the names in both datasets match to ensure I did not mispell any names.

graph.rs is a module containing the Node, Link, and Graph structs. These structs have the same fields as the original json files to make it easy to import the data. This results in some fields that are ultimately redundent - in particular, the "value" and "colour" fields in Node are never used. In addition, graph.rs contains functions for reading a json file into the Graph struct, and for reading a csv file into a HashMap. There is one method for the Graph struct, which returns an adjacency list for every character in order. This method is used in the function to calculate the degrees of separation.

gendercounting.rs contains all of the functions which calculate statitics and degrees of separation. There are six statistics functions - percentwomen, percentwomennodroids, percentffinteractionsall, percentffinteractionswomen, percentmminteractionsall, and percentmminteractionsmen. With these values, you can deduce pretty much every relevant statistic.

makedata.rs generates a graph of 1000 random characters, with random names and random genders, who interact with a random number of other characters with no overlap. The purpose of this module is to show that my code works on larger datasets.

In addition to main{}, main.rs contains the test module, which contains two functions - namesmatch and checkcalc. namesmatch ensures that all of the names that appear in the json files are also in the gender HashMap. It also checks that a graph and hashmap produced by makedata() have matching names. checkcalc verifies that all of the statistics functions, as well as the degree separation function, return the correct output.
