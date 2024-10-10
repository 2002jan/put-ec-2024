# Save this as run_visualization.ps1
$OutputDir = "outputs"
$PythonScript = "visualization/visualize.py"

# Check if the Python script exists
if (-Not (Test-Path $PythonScript)) {
    Write-Host "Error: Python script '$PythonScript' not found."
    exit
}

# Find all .csv files in the output directory and subdirectories
$csvFiles = Get-ChildItem -Path $OutputDir -Recurse -Filter "*.csv"

foreach ($csvFile in $csvFiles) {
    Write-Host "Processing $($csvFile.FullName)"
    python $PythonScript $csvFile.FullName 
}
