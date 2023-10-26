from __future__ import annotations
from typing     import TypedDict
from functools  import reduce

from   torch import Tensor
import torch
import torchvision.transforms as transforms

from image_io import ImageTensor



# This is a type used to store an image's metadata, as required
# per the exercise.
ImageInfo = TypedDict(
	"ImageInfo",
	{
		"filename" : str,
		"height"   : float,
		"width"    : float,
		"r_avg"    : float,
		"g_avg"    : float,
		"b_avg"    : float,
	},
	total = True,
)


def fold_tensor_on_width_and_height(
	filename : str,
	tensor   : ImageTensor,
) -> ImageInfo:
	# get image metadata
	_, height, width = tensor.shape
	pixel_amount     = height * width

	#compute averages by folding along width then height
	inv_pixel_amount = 1. / pixel_amount
	tensor = torch.sum(tensor, dim = 2)
	tensor = torch.sum(tensor, dim = 1)
	r_avg = float(tensor[0] * inv_pixel_amount)
	g_avg = float(tensor[1] * inv_pixel_amount)
	b_avg = float(tensor[2] * inv_pixel_amount)

	#package result neatly
	result : ImageInfo = {
		"filename" : filename,
		"height"   : height,
		"width"    : width,
		"r_avg"    : r_avg,
		"g_avg"    : g_avg,
		"b_avg"    : b_avg,
	}
	return result
