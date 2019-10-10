# Hack-Appleton
The winning project for HackAppleton 2018. An integrated sales analytics platform which generates usable graphs based on custom defined demographics information.

HackAppleton 2018 
Phoenix Kahlo, Anna Arpaci-Dusseau, Langston Nashold, Felix Jiang

The purpose of this website was to live update a series of graphs based on data stored in a backend server about purchases. Correlation analysis is then performed comparing various demographics to purchase date and amount of purchases. In the modern age, this sort of data analysis is critical when analyzing sales in order to maximize effective advertising and more. 

The front end is programmed in HTML with a CSS bootstrap template and JavaScript. For our graphs, we used Chart.js which is a framework for HTML canvas which gives useful graphing tools.

 We dynamically plotted data received from the server in JSON format and then processed the received object to the appropriate format for the Chart.js graph. Additionally, the graphs and the tabs at the top are all created real time based on the data received from the server. This means the charts are automatically scaled if the received JSON file has more demographic categories to plot.

There is also a purchase tab which simulates how data could easily be posted to the server. Realistically, the demographics would be collected from the user’s profile or account and then submitted to the server with the purchase. Here, for demonstration purposes the user simply submits basic demographic info along with amount of products purchased.

The code is very flexible based on whatever data the server has access to and could easily be adjusted to have more categories of demographics and more data points. Given actual customer data about various demographics, we could easily construct the same graphs with real customer purchase data. 

Given more time, additional useful demographics could be collected and analyzed. We planned to additionally collect more rigorous data on region, how long the person spent browsing the website, how expensive the purchases were, what general category the product purchased was in and more. Chart.js makes it very straightforward to create not only line graphs but bar graphs and more. In the future, different types of graphs could be used for other data collection. Additionally, Chart.js makes it very simple to graph multiple charts on the same set of axis, allowing for cross analysis of several factors to discover correlations.

The Java program DataGenerator was used to make sample graphs for our website to try. Part of our website includes showing data about users demographics like age and gender and the time the purchase took place. We decided to have a start time for our website, the time that we started tracking user purchases, to be January 1, 2010. We planned to show the date on the x axis starting from January 1, 2010 to today's date at 11:00am. For the time, the DataGenerator computed a long which was the epoch time in our range (January 1, 2010 to 11:00am today). For age, we had certain age brackets in an array. For gender, we had Male, Female, and Other as options and for our location demographic, we used continents. Realistically, we could have used the 50 states as locations because Kohl’s is mainly known within the United States. However, for ease of implementations, we decided that continents effectively demonstrates the concept. The data generator application randomized 50 different purchases and outputted into a text file in order to be formatted into a JSON object and used on our website. Later, we wanted to test more random points but our first batch of 50 points had required a lot of reformatting because it was missing necessary quotations for each item. We wrote a new output method in the correct format to make it easier to parse. We didn't have enough time to test this new batch of 1000 points. 

The backend is a web server program in Rust. It uses a single-file database which it uses to store a JSON representation of all purchases. It is a multithreaded web server, that provides all HTML files in file tree through GET requests. It also responds to a GET request to `plots.json` with dynamic JSON data calculated from the database representing plots of purchases per day over time, across several demographics, including continent, gender, and age range. The client gets this data and graphs it on charts, where all trends for a particular category of demographics are superimposed. It also responds to POST requests representing a new purchase, which is reacts to by inserting an entry into the database. We did not entirely finish the backend, because of a known glitch in our json serialization library.


## Problems
Working with Chart.js we had to learn new syntax, chart attributes, and how to format our data. We were unfamiliar with the framework and faced several issues properly formatting axis and scaling data.

Additionally, none of us had much experience in connecting servers with front end web pages to exchange JSON files. We had a few problems properly setting up the POST/GET requests and had to experiment with the proper syntax and formatting for this feature. The front end is all written in Javascript, and none of us were especially familiar with javascript. This made debugging the front end difficult as javascript only has run-time errors.

We forgot a charger for one of our laptops. The HackAppleton staff graciously went to the store bought us the right type of charger. However, this charger did not end up charging the laptop, possibly because it was only able to handle the needs of a phone battery. For half of the hackathon, we only had 3 laptops for 4 people. One person ended up starting the report on his phone. Also, one of our members forgot their computer, and had to work off a chromebook for the whole hackathon instead. Although the chromebook had linux installed, it was still a huge pain.

Lastly, the known bug in a JSON library prevented us from effectively utilizing the JSON library. However, we still were able to transfer the json text via Flash drive, and with more time we could use a different library to fully fix the program. 





