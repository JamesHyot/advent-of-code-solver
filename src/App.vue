<script setup lang="ts">
import * as wasm from 'wasm-advent-of-code';
import { useDisplay } from 'vuetify/lib/framework.mjs';

const { xs } = useDisplay();

const textDay1 = ref(wasm.get_day1_default_input());
const resultDay1 = ref('');
const updateResultDay1 = function () {
  resultDay1.value = wasm.get_day1_result(textDay1.value);
};

const textDay2 = ref(wasm.get_day2_default_input());
const resultDay2 = ref('');
const updateResultDay2 = function () {
  resultDay2.value = wasm.get_day2_result(textDay2.value);
};

const textDay3 = ref(wasm.get_day3_default_input());
const resultDay3 = ref('');
const updateResultDay3 = function () {
  resultDay3.value = wasm.get_day3_result(textDay3.value);
};

const textDay4 = ref(wasm.get_day4_default_input());
const resultDay4 = ref('');
const updateResultDay4 = function () {
  resultDay4.value = wasm.get_day4_result(textDay4.value);
};

const textDay5 = ref(wasm.get_day5_default_input());
const resultDay5 = ref('');
const updateResultDay5 = function () {
  resultDay5.value = wasm.get_day5_result(textDay5.value);
};

const textDay6 = ref(wasm.get_day6_default_input());
const resultDay6 = ref('');
const updateResultDay6 = function () {
  resultDay6.value = wasm.get_day6_result(textDay6.value);
};

const textDay7 = ref(wasm.get_day7_default_input());
const resultDay7 = ref('');
const updateResultDay7 = function () {
  resultDay7.value = wasm.get_day7_result(textDay7.value);
};

const textDay8 = ref(wasm.get_day8_default_input());
const resultDay8 = ref('');
const updateResultDay8 = function () {
  resultDay8.value = wasm.get_day8_result(textDay8.value);
};

const textDay9 = ref(wasm.get_day9_default_input());
const resultDay9 = ref('');
const updateResultDay9 = function () {
  resultDay9.value = wasm.get_day9_result(textDay9.value);
};

const theme = ref('dark');
const onThemeClick = function () {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
};
</script>

<template>
  <v-app :theme="theme">
    <v-app-bar app>
      <v-btn href="https://adventofcode.com/">
        Discover Advent of Code
      </v-btn>
      <v-spacer />
      <v-btn :prepend-icon="theme === 'light' ? 'mdi-weather-sunny' : 'mdi-weather-night'" @click="onThemeClick">
        <span v-if="!xs">Toggle theme</span>
      </v-btn>
    </v-app-bar>
    <v-main>
      <v-container fluid>
        <v-row>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card color="indigo" title="Day 1" subtitle="Elves want to know how much calories they have">
              <v-textarea
                v-model="textDay1"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay1 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay1">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 2" color="light-green" subtitle="Learn to cheat at Rock Paper Scissors">
              <v-textarea
                v-model="textDay2"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay2 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay2">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 3" color="amber" subtitle="Oh no! Rucksacks are not packed correctly!">
              <v-textarea
                v-model="textDay3"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay3 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay3">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 4" color="brown" subtitle="Camp cleanup: why you should always let others work for you">
              <v-textarea
                v-model="textDay4"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay4 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay4">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 5" color="indigo" subtitle="Teddy got crane!">
              <v-textarea
                v-model="textDay5"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay5 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay5">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 6" color="light-green" subtitle="I think I intercepted a communication from aliens">
              <v-textarea
                v-model="textDay6"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay6 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay6">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 7" color="amber" subtitle="Where we code a whole filesystem just to find a folder">
              <v-textarea
                v-model="textDay7"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay7 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay7">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 8" color="brown" subtitle="Trees can have a height of 0 ??">
              <v-textarea
                v-model="textDay8"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay8 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay8">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
          <v-col class="mt-2" cols="12" lg="3" md="6">
            <v-card title="Day 9" color="indigo" subtitle="Why am I daydreaming about rope physics ?">
              <v-textarea
                v-model="textDay9"
                label="Input"
                type="text"
                placeholder="The input from Advent of Code"
              />
              <v-card-text>{{ resultDay9 }}</v-card-text>
              <v-card-actions>
                <v-btn @click="updateResultDay9">
                  Get result
                </v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
    <v-footer app>
    <!-- -->
    </v-footer>
  </v-app>
</template>
