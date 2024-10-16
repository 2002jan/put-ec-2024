# Save this as run_visualization.ps1
$OutputDir = "outputs"
$PythonScript = "visualization/visualize.py"
$dataset = "data/TSPB.csv"

# Get outputdir  and dataset from commandline
if ($args.Length -eq 2) {
    $OutputDir = $args[0] # directory containing the .csv files
    $dataset = $args[1]   # dataset for plotting (data\TSPA, data\TSPB)
}

# Check if the Python script exists
if (-Not (Test-Path $PythonScript)) {
    Write-Host "Error: Python script '$PythonScript' not found."
    exit
}

# Find all .csv files in the output directory and subdirectories
$csvFiles = Get-ChildItem -Path $OutputDir -Recurse -Filter "*.csv"

# create plots directory if it does not exist and make a subdir with timestamp for plots
$plotsDir = Join-Path $OutputDir "plots"
if (-Not (Test-Path $plotsDir)) {
    Write-Host "Creating plots directory: $plotsDir"
    New-Item -ItemType Directory -Path $plotsDir
}

foreach ($csvFile in $csvFiles) {
    Write-Host "Processing $($csvFile.FullName)"
    python $PythonScript $csvFile.FullName $dataset $plotsDir
}
