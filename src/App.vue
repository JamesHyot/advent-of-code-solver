<script setup lang="ts">
import * as wasm from 'wasm-advent-of-code';

const textDay1 = ref('');
const resultDay1 = ref('');
const updateResultDay1 = function () {
  resultDay1.value = wasm.get_day1_result(textDay1.value);
};

const textDay2 = ref('');
const resultDay2 = ref('');
const updateResultDay2 = function () {
  resultDay2.value = wasm.get_day2_result(textDay2.value);
};

const textDay3 = ref('');
const resultDay3 = ref('');
const updateResultDay3 = function () {
  resultDay3.value = wasm.get_day3_result(textDay3.value);
};

const textDay4 = ref('');
const resultDay4 = ref('');
const updateResultDay4 = function () {
  resultDay4.value = wasm.get_day4_result(textDay4.value);
};

const textDay5 = ref('');
const resultDay5 = ref('');
const updateResultDay5 = function () {
  resultDay5.value = wasm.get_day5_result(textDay5.value);
};

const textDay6 = ref('');
const resultDay6 = ref('');
const updateResultDay6 = function () {
  resultDay6.value = wasm.get_day6_result(textDay6.value);
};

const textDay7 = ref('');
const resultDay7 = ref('');
const updateResultDay7 = function () {
  resultDay7.value = wasm.get_day7_result(textDay7.value);
};

const textDay8 = ref('');
const resultDay8 = ref('');
const updateResultDay8 = function () {
  resultDay8.value = wasm.get_day8_result(textDay8.value);
};

const theme = ref('dark');
const onThemeClick = function () {
  theme.value = theme.value === 'dark' ? 'light' : 'dark';
};
</script>

<template>
  <v-app :theme="theme">
    <!-- <v-navigation-drawer app /> -->
    <v-app-bar app>
      <v-spacer />
      <v-btn :prepend-icon="theme === 'light' ? 'mdi-weather-sunny' : 'mdi-weather-night'" @click="onThemeClick">
        Toggle theme
      </v-btn>
    </v-app-bar>
    <v-main>
      <v-container fluid>
        <v-row>
          <v-col class="mt-2" cols="3">
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
          <v-col class="mt-2" cols="3">
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
          <v-col class="mt-2" cols="3">
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
          <v-col class="mt-2" cols="3">
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
          <v-col class="mt-2" cols="3">
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
          <v-col class="mt-2" cols="3">
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
          <v-col class="mt-2" cols="3">
            <v-card title="Day 7" color="amber">
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
          <v-col class="mt-2" cols="3">
            <v-card title="Day 8" color="brown">
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
        </v-row>
      </v-container>
    </v-main>
    <v-footer app>
    <!-- -->
    </v-footer>
  </v-app>
</template>
