# nagamochi

![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/Lucky3028/nagamochi/Build%20and%20Release/main?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/Lucky3028/nagamochi?style=for-the-badge)

## 説明

Ubuntu用のバッテリ容量を通知してくれるプログラムです。設定ファイルに基づいて、一定のバッテリ容量・条件を満たした場合、音声ファイルを再生しデスクトップ通知を行います。

## 使い方

`$ ./nagamochi`

## 設定

`nagamochi.yml`を以下のいずれの場所に作成してください。

* ~/.config/nagamochi/nagamochi.yml
* ~/nagamochi.yml
* ./nagamochi.yml

## nagamochi.yml（設定ファイル）の書き方

[こちらのファイル](./src/tests/configs/general_config.yml)を参照してください。
