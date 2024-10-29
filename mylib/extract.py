"""
Extract a dataset from a URL like Kaggle or data.gov. 
JSON or CSV formats tend to work well
"""

import os
import requests
import pandas as pd

def extract(
    url="https://raw.githubusercontent.com/cpyang123/DE-W5/refs/heads/main/train.csv",
    file_path="data/housing_data.csv",
    directory="data",
):
    """Extract a url to a file path"""
    if not os.path.exists(directory):
        os.makedirs(directory)
    with requests.get(url) as r:
        with open(file_path, "wb") as f:
            f.write(r.content)

    df = pd.read_csv(file_path)

    df_subset = df

    df_subset.to_csv(file_path, index=False)
    return file_path