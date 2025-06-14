#!/bin/bash

# 批量复制 TODO.md 文件到 src/components 目录

components=("affix" "alert" "anchor" "auto_complete" "avatar" "badge" "breadcrumb" "button" "calendar" "card" "carousel" "cascader" "checkbox" "collapse" "comment" "config_provider" "date_picker" "descriptions" "divider" "drawer" "dropdown" "empty" "flex" "form" "grid" "icon" "image" "input" "input_number" "layout" "list" "mentions" "menu" "message" "modal" "notification" "pagination" "popconfirm" "popover" "progress" "qr_code" "radio" "rate" "result" "segmented" "select" "skeleton" "slider" "space" "spin" "statistic" "steps" "switch" "table" "tabs" "tag" "timeline" "tooltip" "tour" "transfer" "tree" "tree_select" "typography" "upload" "watermark")

for component in "${components[@]}"; do
  src_file="/Users/zxl/Sites/rust/UI/ant-design-dioxus/src/components/$component/TODO.md"
  if [ -f "$src_file" ]; then
    target_dir="/Users/zxl/Sites/rust/UI/ant-design-dioxus/output/components/$component"
    mkdir -p "$target_dir"
    cp "$src_file" "$target_dir/"
    echo "Copied TODO.md for $component"
  else
    echo "No TODO.md found for $component"
  fi
done

echo "TODO.md files copy completed!"
