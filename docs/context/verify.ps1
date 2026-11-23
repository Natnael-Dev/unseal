$ErrorActionPreference='Stop'
$root=(Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$files=Get-ChildItem "$root/docs/context" -Filter '*.md'
$ids=@()
$linkCount=0
foreach ($file in $files) {
 $body=[IO.File]::ReadAllText($file.FullName)
 $ids+=@([regex]::Matches($body,'(?m)^## (\d+)\s*[.·]') | ForEach-Object { [int]$_.Groups[1].Value })
 foreach($link in [regex]::Matches($body,'\]\(([^ )]+)\)')){
  $target=$link.Groups[1].Value
  if ($target -match '^(https?:|#|mailto:)') { continue }
  $target=$target.Split('#')[0]
  if (!(Test-Path -LiteralPath (Join-Path $file.DirectoryName $target))) {throw "Broken local link $target in $($file.Name)"}
  $linkCount++
 }
}
foreach($n in 1..70){if(@($ids|Where-Object {$_ -eq $n}).Count -ne 1){throw "Section $n missing or duplicated"}}
$product=[IO.File]::ReadAllText("$root/docs/context/product.md")
$decisions=@([regex]::Matches($product,'(?m)^\| (\d+) \|')|ForEach-Object{[int]$_.Groups[1].Value})
foreach($n in 1..35){if(@($decisions|Where-Object {$_ -eq $n}).Count -ne 1){throw "Decision $n missing or duplicated"}}
Write-Output "70 numbered sections; 35 numbered decisions; $linkCount local document links resolved; $($files.Count) Markdown files."
git -C $root diff --check c383318
exit $LASTEXITCODE
