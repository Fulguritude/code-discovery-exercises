# Big Data & Analytics Advanced

# Final exam (2023-10-23, duration <3h15)

## General instructions

**Each exercise should last approximately 50 minutes to 1 hour**. Each of the exercises could use a lot more time and care. That's not the goal here. It is better to set a strict timer, get to the essentials, and not expand too much.

**Read the subject as a whole first, and allow time for rereading before sending your work.** After 3 hours of examination, you will have around ten minutes to submit your work on Teams via the "Assignments" feature. This work will consist of an archive (`.zip`, `.tar`, etc) containing 3 folders (one per exercise), each containing your textual answers, any photos or images of diagrams (with captions), or your code files. Make sure your code files, in particular, are well organized, so that your code can run as soon as I open the archive and run the command `python3 main.py` in the correct folder; or so that a variable `DATASET_FILEPATH` is clearly present, so that I can put the path of mine.

If you work does not follow these instructions, I may withdraw points. Same, if your text is really illegible (too many language mistakes).

**Please note, if you send a corrupted archive and leave the room, or exceed the authorized deadlines, your exam will be counted as invalid (zero!), and you will be invited to the remedials, which are not necessarily nicer... That would be a shame. 🙃** I will gradually confirm the valid archives I received, so that you can go home without stress.

A priori, the exam is based on 23 points, with 3 bonus points. If certain exercises prove too difficult for the whole class, the scale may be revised given the short time between this module and the exam.

Run the command to download the Python libraries (if it is not already done) and download the flower dataset ( https://www.robots.ox.ac.uk/~vgg/data/flowers/102/index.html ) at the start of the exam so as not to waste time.



## Exercise I) Knowledge restitution (8 points)

**This exercise aims to assess the level of abstraction and knowledge in your understanding of big data.**

### Statement

Describe the different stages of a data pipeline.

For each step:
   - name this step, explain this (these) name(s)
   - define this step
   - talk about the specific big data issues for this step (fundamental concepts and essential considerations to take into account)
   - describe the type of task or role that must be carried out to manage this stage
   - give one or two concrete examples

*NB: our friends, (understandable) diagrams, are welcome.*

### Detailed scale

- a coherent and complete division and definition of steps (2 points)
- having spoken about the specific issues at each stage (1.5 points)
- description of the type of task and role that must be carried out to manage each step (0.75 points)
- one or two concrete examples of this step for a chosen domain (0.75 points)
- a more detailed description of certain slightly more complex steps (1 point)
- particularly clear explanations (1 point)
- highlighting the problems specific to big data, on the notion of "data pipeline" itself (0.5 points)
- an explanation of the limitations of your chosen pipeline model (0.5 points)



## Exercise II) Data manipulation and analysis (8 points)

**This exercise consists of studying a relatively commonplace, relatively large, end-to-end dataset.**

### States

You are provided with a folder containing a dataset of image files of flowers. This folder corresponds to a "large" unstructured dataset that you need to process. With the Python 3 programming language, using the usual data and image processing libraries, you must create a data table containing simple statistics on these images, and display these statistics in a plot. This table will have the following attributes (and in this order):

   - Filename: the name of the initial file without extension.
   - Height: the height of the image.
   - Width: the width of the image.
   - Average_Red: the average red value for a pixel of this image.
   - Average_Green: the average green value for a pixel of this image.
   - Average_Blue: the average blue value for a pixel of this image.

You will also need to provide something to visualize each image as a point in a 3D frame. This benchmark will have as axes (named “Red”, “Green” and “Blue”) the 3 average color values. You will need to do a summary analysis of this data (either as a comment in the code, or in a separate `.txt` file) and save this table in `.tsv` form. Your script, when executed, should automatically produce an `imageinfo.tsv` file, and display the plot. Your code can be split into as many files as you want; however, your software must be executable *via* the `python3 main.py` command.

NB: You will need to code your names in English, not another language.

NB: We recommend the pandas, PIL, pytorch (or numpy), matplotlib (with seaborn or holoviews, possibly) libraries, and Python's standard library.

NB: Points will be awarded for the appropriate division of code into simple, typed and composable functions; as well as on the quality of code readability.

NB: if you produce things that surprise me in their quality (really nice plot, interesting analysis, additional statistics), bonus points are possible. Don't count on that though: focus on the scale.

### Detailed scale
   - reading the data (0.5 points)
   - transforming the data (2 points)
   - producing statistics on the data (1.5 points)
   - producing a standard dataset in its own file (1 point)
   - producing a readable visualization and a summary explanation of the data (1 point)
   - readability and quality of the code (2 points)



## Exercise III) System design (7 points)

**This exercise is a design exercise. It tests your adaptability and mastery, when it comes to a concrete big data problem.**

### States

A client asks you to design a computer system for them. This client is a massive, highly secure military installation. This facility's sole activity is the storage and transportation of weapons and personnel. Staff generally stay from several days to several months.

The client wants to develop and maintain the entire system and software in-house for security reasons. You are its only contractor. Any inconsistency error in the database should be absolutely avoided, as it is a risk of national scale. All physical communication with the outside world takes place through trucks through a few monitored gates (some reserved for entrances, others for exits). A camera- and alarm-based surveillance system is active throughout the base. The computer system is otherwise not connected to the outside: accredited people contact the General Staff only through mobile phones on a specialized network (mobiles which are the only thing you will not need take care of). Ideally, system performance should enable immediate detection and response to problems (within the minute timeframe).

The number of weapons stored at any time is estimated at several million, the number of soldiers present roughly tens of thousands, and the daily movement is counted in tens of thousands of weapons and thousands of soldiers. Each truck moves on average around fifty soldiers or a few hundred weapons.

Think about the different data that you will have to model, the types of individuals who will have to interact with the system, the security issues, the efficient operation of the computer system and its link with the physical system, etc. The description of the domain is left quite vague (and deliberately incomplete!) to leave some room for your creativity. (Often, the client's needs are not necessarily immediately understood by the engineer, but you still have to make the client interested, and show that you have an idea of their specific issues.)

Propose an architecture for this system, and explain how it responds to the various *types* of issues that need to be resolved, while presenting the questions you asked yourself while designing, and justifying your train of thought.


### Detailed scale

   - ability to ask oneself the right questions for systems engineering and database architecture (1 point)
   - ability to model a domain in the form of databases in a system (1.5 points)
   - presentation of the system as a whole (inputs and outputs, hardware and software) (1.5 points)
   - understanding the limits (tradeoffs) of the answers provided (1 point)
   - quality of communication of the design to the client (clear and simple diagrams, sentences and ideas) (1 point)
   - creativity and quality of the solution (1 point)
