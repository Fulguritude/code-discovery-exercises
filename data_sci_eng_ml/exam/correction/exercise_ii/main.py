"""
1) trouver dataset (Google)
2) extraire les données (librairie standard de python)
3) les formatter (numpy, pytorch, tensorflow)
4) traitement de données dessus (numpy, pytorch, scikit-learn)
5) visualiser les résultats (matplotlib, seaborn, holoviews)
"""


#Always separate imports into 3 sections: stdlib, 3rd party dependencies, local sources (in that order)
from __future__ import annotations
from typing     import Optional

import torch
import numpy             as np
import seaborn           as sns
import pandas            as pd
import matplotlib        as mpl
import matplotlib.pyplot as plt

from image_io import (
	read_imagetensor,
	read_image_dataset,
)
from transforms import (
	ImageInfo,
	fold_tensor_on_width_and_height,
)


# This is a type alias that we
ImageDataDict = dict[int, ImageInfo]


# Constants
DATASET_FILEPATH = "/home/fulguritude/ProfessionalWork/Enseignement/Estiam/exam/correction/ExerciceII/jpg/"
READFILE_LIMIT   = 10


### Utils

def extract_filename(filepath: str) -> str:
	filename        = filepath.split("/")[-1]
	filename_no_ext = ".".join(filename.split(".")[:-1])
	return filename_no_ext


def extract_index(filepath: str) -> int:
	filename = extract_filename(filepath)
	index    = int(filename[6:])
	return index



### Substeps for main 

def read_dataset_average_colors(
	dataset_filepath : str           = DATASET_FILEPATH,
	readfile_limit   : Optional[int] = None,
) -> ImageDataDict:
	filename_tensors_dict = read_image_dataset(dataset_filepath, readfile_limit)
	filename_values_dict  = {
		extract_index(filename): fold_tensor_on_width_and_height(extract_filename(filename), tensor)
		for filename, tensor in filename_tensors_dict.items()
	}
	return filename_values_dict


def plot_image_averages(filename_values_dict: dict[int, ImageInfo]):
	# This function prepares the precomputed data for visualisation.
	# It also, helpfully, represents each point in the scatterplot via
	# the color value that defines it.
	inv_256 = 1. / 256.

	r_avg_values = [value["r_avg"] for value in filename_values_dict.values()]
	g_avg_values = [value["g_avg"] for value in filename_values_dict.values()]
	b_avg_values = [value["b_avg"] for value in filename_values_dict.values()]
	color_values = [
		(
			value["r_avg"] * inv_256,
			value["g_avg"] * inv_256,
			value["b_avg"] * inv_256,
		)
		for value in filename_values_dict.values()
	]

	fig = plt.figure()
	ax = fig.add_subplot(111, projection = "3d")
	ax.scatter(
		r_avg_values,
		g_avg_values,
		b_avg_values,
		c = color_values,
	)
	ax.set_xlabel("Red"   ) #, fontsize=20, rotation=150
	ax.set_ylabel("Green" )
	ax.set_zlabel("Blue"  )
	plt.title("Distribution of average image RGB colors per image")
	plt.show()


def produce_table(data_dict: ImageDataDict) -> None:
	df = pd.DataFrame(data_dict)  #type:ignore
	df = df.transpose()
	df.to_csv(
		"imageinfo.tsv",
		index  = True,
		header = True,
		sep    = "\t",
	)



# Main and data analysis

if __name__ == "__main__":
	image_stats = read_dataset_average_colors() #, READFILE_LIMIT)
	plot_image_averages (image_stats)
	produce_table       (image_stats)

	"""
	Analysis of the 3D plot:
	The first thing we notice is that almost the full range of color values (from 0 to 255) is used. 

	Secondly, we notice is that there seems to be a pretty strong correlation between red, green and blue.
	In image parlance, this correlation illustrates the effect that photo brightness/luminance has over
	the average of each color.

	What we also notice is that variations from this tendency tend to be mostly in the red region. There
	don't seem to be many "very" blue or green flowers. Perhaps this hints at some evolutionary trait, since
	red contrasts more with green and blue, than green and blue do together, in nature.

	Plotting the averages as HSL/HSB would probably be more informative here.
	Further analyses of this kind could also try to normalize the brightness value, and compare images after
	this normalization, but on a 2D plot.
	"""
